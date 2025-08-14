//! Session service - Handles session management logic

use std::sync::Arc;
use chrono::{Utc, Duration};
use crate::{
    database::Database,
    repository::session::SessionRepository,
    repository::auth::AuthRepository,
    service::oauth::SimplifiedOAuthService,
    utils::error::AppError,
    repository::session::UserSessionData,
};

/// Session service for handling session operations
pub struct SessionService {
    database: Arc<Database>,
}

impl SessionService {
    /// Create new session service
    pub fn new(database: Arc<Database>) -> Self {
        Self {
            database,
        }
    }
    
    /// Get session by ID
    pub async fn get_session(&self, session_id: &str) -> Result<Option<crate::entity::UserSession>, AppError> {
        let session_repository = SessionRepository::new(Arc::clone(&self.database));
        session_repository.get_session(session_id).await
    }
    
    /// Create new session
    pub async fn create_session(&self, session: crate::entity::UserSession) -> Result<crate::entity::UserSession, AppError> {
        let session_repository = SessionRepository::new(Arc::clone(&self.database));
        session_repository.create_session(session).await
    }
    
    /// Delete session
    pub async fn delete_session(&self, session_id: &str) -> Result<(), AppError> {
        let session_repository = SessionRepository::new(Arc::clone(&self.database));
        session_repository.delete_session(session_id).await
    }

    /// Update (or attach) Axiom tokens onto existing user session data
    pub async fn update_axiom_tokens(
        &self,
        session_id: &str,
        access_token: Option<String>,
        refresh_token: Option<String>,
        user_id: Option<String>,
        connected_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), AppError> {
        let repo = SessionRepository::new(Arc::clone(&self.database));
        if let Some(session) = repo.get(session_id).await? {
            let ttl = session.remaining_ttl();
            let mut data: UserSessionData = serde_json::from_str(&session.data)
                .map_err(|e| AppError::SerializationError(format!("Failed to parse session data: {}", e)))?;
            data.axiom_access_token = access_token;
            data.axiom_refresh_token = refresh_token;
            data.axiom_user_id = user_id;
            data.axiom_connected_at = Some(connected_at);
            repo.update_user_session(session_id, data, ttl).await?;
        }
        Ok(())
    }

    /// Get strongly typed user session data (extended) for convenience
    pub async fn get_user_session_data(&self, session_id: &str) -> Result<Option<UserSessionData>, AppError> {
        let repo = SessionRepository::new(Arc::clone(&self.database));
        repo.get_user_session(session_id).await
    }

    /// Get latest active session for a user email
    pub async fn get_latest_session_by_email(&self, user_email: &str) -> Result<Option<UserSessionData>, AppError> {
        let repo = SessionRepository::new(Arc::clone(&self.database));
        match repo.find_latest_by_user_email(user_email).await? {
            Some((_session, data)) => Ok(Some(data)),
            None => Ok(None),
        }
    }

    /// Lookup latest Google OAuth access token via tokens table (delegates to AuthRepository)
    pub async fn get_latest_google_access_token(&self, user_email: &str) -> Result<Option<String>, AppError> {
        let auth_repo = AuthRepository::new(Arc::clone(&self.database));
        auth_repo.get_latest_google_access_token(user_email).await
    }

    /// Derive user_id (UUID) from a session cookie value (user_id:token_id) without DB verification
    pub fn parse_user_id_from_cookie(&self, session_cookie: &str) -> Option<uuid::Uuid> {
        let parts: Vec<&str> = session_cookie.split(':').collect();
        if parts.len() == 2 { uuid::Uuid::parse_str(parts[0]).ok() } else { None }
    }

    /// Lookup latest Google access token by user_id with automatic refresh if expired
    pub async fn get_latest_google_access_token_by_user_id(&self, user_id: uuid::Uuid) -> Result<Option<String>, AppError> {
        let auth_repo = AuthRepository::new(Arc::clone(&self.database));
        
        // First try to get a valid access token
        match auth_repo.get_latest_google_access_token_by_user_id(user_id).await? {
            Some(token) => {
                worker::console_log!("✅ Found valid access token for user {}", user_id);
                Ok(Some(token))
            }
            None => {
                worker::console_log!("🔄 No valid access token found, attempting refresh for user {}", user_id);
                
                // Try to get refresh token
                match auth_repo.get_latest_google_refresh_token_by_user_id(user_id).await? {
                    Some(refresh_token) => {
                        worker::console_log!("🔄 Found refresh token, refreshing access token");
                        
                        // Create OAuth service for token refresh
                        let oauth_service = SimplifiedOAuthService::new(
                            std::env::var("GOOGLE_CLIENT_ID").unwrap_or_default(),
                            std::env::var("GOOGLE_CLIENT_SECRET").unwrap_or_default(),
                            std::env::var("GOOGLE_REDIRECT_URI").unwrap_or_default(),
                        );
                        
                        // Refresh the access token
                        match oauth_service.refresh_access_token(&refresh_token).await {
                            Ok((new_access_token, expires_in)) => {
                                let expires_at = Utc::now() + Duration::seconds(expires_in);
                                
                                // Store the new access token
                                auth_repo.update_access_token(user_id, &new_access_token, expires_at).await?;
                                
                                worker::console_log!("✅ Successfully refreshed and stored new access token");
                                Ok(Some(new_access_token.to_string()))
                            }
                            Err(e) => {
                                worker::console_log!("❌ Failed to refresh access token: {}", e);
                                Err(e)
                            }
                        }
                    }
                    None => {
                        worker::console_log!("❌ No refresh token found for user {}", user_id);
                        Ok(None)
                    }
                }
            }
        }
    }
}

/// Extract session token from HTTP cookies (standalone utility function)
pub fn extract_session_token_from_cookies(parts: &axum::http::request::Parts) -> Option<String> {
    parts
        .headers
        .get(axum::http::header::COOKIE)?
        .to_str()
        .ok()?
        .split(';')
        .find_map(|cookie| {
            let cookie = cookie.trim();
            if cookie.starts_with("session_id=") {
                cookie.split('=').nth(1).map(|s| s.trim().to_string())
            } else {
                None
            }
        })
}