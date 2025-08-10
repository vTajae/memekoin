//! WASM-compatible session middleware
//! 
//! This replaces tower-sessions with a WASM-compatible implementation
//! that uses chrono instead of the time crate.

use std::sync::Arc;
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
};
use serde_json::Value;
use std::collections::HashMap;
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;
use worker::console_log;

use crate::service::session::SessionService;

/// WASM-compatible session that avoids time crate
#[derive(Clone, Debug)]
pub struct WasmSession {
    id: String,
    data: HashMap<String, Value>,
    modified: bool,
    session_service: Arc<SessionService>,
}

impl WasmSession {
    /// Create a new session
    pub fn new(session_service: Arc<SessionService>) -> Self {
        let id = Uuid::new_v4().to_string();
        Self {
            id,
            data: HashMap::new(),
            modified: false,
            session_service,
        }
    }

    /// Load existing session from storage
    pub async fn load(id: String, session_service: Arc<SessionService>) -> Option<Self> {
        match session_service.session_repo.get(&id).await {
            Ok(Some(db_session)) => {
                // Check expiry using chrono
                let now_timestamp = chrono::Utc::now().timestamp();
                if db_session.expires <= now_timestamp {
                    // Session expired
                    let _ = session_service.session_repo.delete(&id).await;
                    return None;
                }

                // Decode session data
                match serde_json::from_str::<HashMap<String, Value>>(&db_session.data) {
                    Ok(data) => Some(Self {
                        id,
                        data,
                        modified: false,
                        session_service,
                    }),
                    Err(_) => None,
                }
            }
            _ => None,
        }
    }

    /// Get session ID
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Insert a value into the session
    pub fn insert<T: serde::Serialize>(&mut self, key: &str, value: T) -> Result<(), serde_json::Error> {
        let json_value = serde_json::to_value(value)?;
        self.data.insert(key.to_string(), json_value);
        self.modified = true;
        Ok(())
    }

    /// Get a value from the session
    pub fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Option<T> {
        self.data.get(key)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }

    /// Remove a value from the session
    pub fn remove(&mut self, key: &str) -> Option<Value> {
        self.modified = true;
        self.data.remove(key)
    }

    /// Save session to storage
    pub async fn save(&self) -> Result<(), String> {
        if !self.modified {
            return Ok(());
        }

        let data_json = serde_json::to_string(&self.data)
            .map_err(|e| format!("Failed to serialize session data: {}", e))?;

        // Use 24 hour session duration
        let ttl_seconds = 24 * 60 * 60; // 24 hours
        
        let session = crate::repository::session::Session::new(
            self.id.clone(),
            data_json,
            ttl_seconds,
        );

        self.session_service
            .session_repo
            .create(session)
            .await
            .map_err(|e| format!("Failed to save session: {}", e))?;

        console_log!("🔐 WASM: Session saved: {}", self.id);
        Ok(())
    }

    /// Clear all session data
    pub fn clear(&mut self) {
        self.data.clear();
        self.modified = true;
    }
}

/// Extract session from request
#[async_trait::async_trait]
impl<S> FromRequestParts<S> for WasmSession
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Get cookies from request
        let cookies = Cookies::from_request_parts(parts, _state).await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

        // Try to get session ID from cookie
        if let Some(session_cookie) = cookies.get("wasm_session_id") {
            let session_id = session_cookie.value();
            
            // Get session service from extensions (set by middleware)
            if let Some(session_service) = parts.extensions.get::<Arc<SessionService>>() {
                // Try to load existing session
                if let Some(session) = WasmSession::load(session_id.to_string(), session_service.clone()).await {
                    return Ok(session);
                }
            }
        }

        // Create new session if no valid existing session
        if let Some(session_service) = parts.extensions.get::<Arc<SessionService>>() {
            Ok(WasmSession::new(session_service.clone()))
        } else {
            Err(StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
    }
}

/// Middleware to add session service to request extensions and handle session cookies
pub async fn wasm_session_middleware<B>(
    mut req: axum::http::Request<B>,
    next: axum::middleware::Next<B>,
) -> axum::response::Response {
    let mut response = next.run(req).await;
    
    // TODO: Handle session cookie setting in the response
    // This could involve:
    // 1. Extracting session from response extensions if modified
    // 2. Setting the wasm_session_id cookie with the session ID
    // 3. Setting appropriate cookie attributes (HttpOnly, Secure, SameSite)
    
    response
}

/// Layer factory for the WASM session middleware
pub fn wasm_session_layer(session_service: Arc<SessionService>) -> axum::middleware::AddExtensionLayer<Arc<SessionService>> {
    axum::middleware::AddExtensionLayer::new(session_service)
}