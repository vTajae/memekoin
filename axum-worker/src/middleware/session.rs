//! Simple header-based session management
//!
//! WASM-compatible session management using HTTP headers and direct database storage

use axum::http::{HeaderMap, HeaderValue};
use worker::console_log;

use crate::{state::AppState, utils::error::AppError};

/// Simple session response with headers
#[derive(Debug, Clone)]
pub struct SimpleSessionResponse {
    pub user_id: String,
    pub token_id: String,
    pub session_id: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

impl SimpleSessionResponse {
    /// Create a new session response
    pub fn new(user_id: String, token_id: String) -> Self {
        let session_id = uuid::Uuid::new_v4().to_string();
        let expires_at = chrono::Utc::now() + chrono::Duration::hours(24);
        
        Self {
            user_id,
            token_id,
            session_id,
            expires_at,
        }
    }

    /// Create Set-Cookie headers for the response
    pub fn create_headers(&self, secure: bool) -> HeaderMap {
        let mut headers = HeaderMap::new();
        
        // Create session cookie value combining user_id and token_id
        let session_value = format!("{}:{}", self.user_id, self.token_id);
        
        // Create Set-Cookie header with proper attributes
        // Note: Removing HttpOnly flag so frontend can read session cookie for auth state
        let cookie_value = if secure {
            format!(
                "session_id={}; Path=/; Secure; SameSite=Lax; Max-Age={}",
                session_value,
                24 * 60 * 60 // 24 hours in seconds
            )
        } else {
            format!(
                "session_id={}; Path=/; SameSite=Lax; Max-Age={}",
                session_value,
                24 * 60 * 60 // 24 hours in seconds
            )
        };
        
        if let Ok(header_value) = HeaderValue::from_str(&cookie_value) {
            headers.insert("Set-Cookie", header_value);
        }
        
        console_log!("🔐 SIMPLE-SESSION: Created session headers for user: {}", self.user_id);
        headers
    }

    /// Check if session is valid (not expired)
    pub fn is_valid(&self) -> bool {
        chrono::Utc::now() < self.expires_at
    }
}

/// No session layer needed - we use cookie extractors directly in handlers
pub async fn simple_session_setup(_state: &AppState) -> Result<(), AppError> {
    console_log!("🔐 SIMPLE-SESSION: Using cookie-based session management");
    Ok(())
}
