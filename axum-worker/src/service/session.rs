//! Session Service - Complete session management for Phase 2 authentication

// use std::collections::HashMap; // unused in header-based session approach
use std::net::IpAddr;
use std::sync::Arc;
use tracing::{warn, error};
use uuid::Uuid;
use worker::console_log;

// Simple header-based session management - no tower-sessions dependency
// use serde_json::Value; // unused in header-based session approach

use crate::{
    error::AppError,
    entity::{User, UserSession},
    repository::{
        user::UserRepository,
        session::SessionRepository,
    },
    dto::oauth::{CreateUserSessionRequest, UserSessionResponse},
};

/// Session service for managing user sessions and authentication state
#[derive(Clone)]
pub struct SessionService {
    pub(crate) session_repo: Arc<SessionRepository>,
    pub(crate) user_repo: Arc<UserRepository>,
    pub(crate) session_duration: chrono::Duration,
    pub(crate) cleanup_interval: chrono::Duration,
}

impl std::fmt::Debug for SessionService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SessionService")
            .field("session_duration", &self.session_duration)
            .field("cleanup_interval", &self.cleanup_interval)
            .finish()
    }
}

impl SessionService {
    /// Create new session service
    pub fn new(
        session_repo: Arc<SessionRepository>,
        user_repo: Arc<UserRepository>,
    ) -> Self {
        Self {
            session_repo,
            user_repo,
            session_duration: chrono::Duration::hours(24), // 24 hour sessions
            cleanup_interval: chrono::Duration::hours(1),   // Cleanup every hour
        }
    }

    /// Default session duration (24 hours)
    /// Used for cookie-based session management
    pub fn default_session_duration() -> chrono::Duration {
        chrono::Duration::hours(24)
    }

    /// Create new session service with custom durations
    pub fn with_durations(
        session_repo: Arc<SessionRepository>,
        user_repo: Arc<UserRepository>,
        session_duration: chrono::Duration,
        cleanup_interval: chrono::Duration,
    ) -> Self {
        Self {
            session_repo,
            user_repo,
            session_duration,
            cleanup_interval,
        }
    }

    /// Create user session from OAuth authentication
    pub async fn create_user_session(
        &self,
        user: User,
        _access_token: String,
        _refresh_token: Option<String>,
        _token_expires_at: chrono::DateTime<chrono::Utc>,
        user_agent: Option<String>,
        ip_address: Option<IpAddr>,
    ) -> Result<UserSession, AppError> {
        let session_id = Uuid::new_v4();
        let user_id = user.id();
        
        console_log!("Creating user session for user: {} ({})", user.email(), user_id);

        // Create the session with our aligned entity
        let expires_at = chrono::Utc::now() + self.session_duration;
        let user_agent_str = user_agent.map(|ua| ua.to_string());
        let ip_str = ip_address.map(|ip| ip.to_string());
        
        let session = UserSession::new(
            session_id.to_string(),
            user_id,
            expires_at,
            user_agent_str,
            ip_str,
        );

        // Store session
        self.session_repo.create_session(session.clone()).await
            .map_err(|e| {
                error!("Failed to create session in repository: {}", e);
                AppError::DatabaseError(format!("Failed to create session: {}", e))
            })?;

        console_log!("Successfully created user session: {}", session_id);
        
        Ok(session)
    }

    /// Create session from request DTO
    pub async fn create_session_from_request(
        &self,
        request: CreateUserSessionRequest,
    ) -> Result<UserSessionResponse, AppError> {
        // Parse IP address if provided
        let ip_address = match request.ip_address {
            Some(ip_str) => {
                ip_str.parse::<IpAddr>()
                    .map_err(|e| AppError::ValidationError(format!("Invalid IP address: {}", e)))?
                    .into()
            }
            None => None,
        };

        // Get user from repository
        let user = self.user_repo.get_user_by_id(&request.user_id.to_string()).await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        // Create session
        let session = self.create_user_session(
            user,
            request.access_token,
            request.refresh_token,
            request.token_expires_at,
            request.user_agent,
            ip_address,
        ).await?;

        Ok(UserSessionResponse::from_session(&session))
    }

    /// Validate session and return user if valid
    pub async fn validate_session(
        &self,
        session_id: &str,
    ) -> Result<(User, UserSession), AppError> {
        let session_uuid = Uuid::parse_str(session_id)
            .map_err(|_| AppError::ValidationError("Invalid session ID format".to_string()))?;

        // Get session from repository
        let session = self.session_repo.get_session(&session_uuid.to_string()).await?
            .ok_or_else(|| AppError::AuthenticationError("Session not found".to_string()))?;

        // Check if session is expired
        if session.is_expired() {
            console_log!("Session expired: {}", session_id);
            
            // Clean up expired session
            let _ = self.session_repo.delete_session(session_id).await;
            
            return Err(AppError::AuthenticationError("Session expired".to_string()));
        }

        // Get user
        let user = self.user_repo.get_user_by_id(&session.user_id().to_string()).await?
            .ok_or_else(|| AppError::AuthenticationError("User not found for session".to_string()))?;

        console_log!("Session validated successfully for user: {}", user.email());
        
        Ok((user, session))
    }

    /// Refresh session (extend expiry)
    pub async fn refresh_session(
        &self,
        session_id: &str,
    ) -> Result<UserSession, AppError> {
        let session_uuid = Uuid::parse_str(session_id)
            .map_err(|_| AppError::ValidationError("Invalid session ID format".to_string()))?;

        // Get current session
        let mut session = self.session_repo.get_session(&session_uuid.to_string()).await?
            .ok_or_else(|| AppError::AuthenticationError("Session not found".to_string()))?;

        // Refresh the session
        session.refresh(self.session_duration)
            .map_err(|e| AppError::AuthenticationError(format!("Failed to refresh session: {}", e)))?;

        // Update in repository
        self.session_repo.update_session(&session_uuid.to_string(), session.clone()).await?;

        console_log!("Session refreshed successfully: {}", session_id);
        
        Ok(session)
    }

    /// End session (logout)
    pub async fn end_session(
        &self,
        session_id: &str,
    ) -> Result<(), AppError> {
        let session_uuid = Uuid::parse_str(session_id)
            .map_err(|_| AppError::ValidationError("Invalid session ID format".to_string()))?;

        console_log!("Ending session: {}", session_id);

        // Delete session from repository
        self.session_repo.delete_session(&session_uuid.to_string()).await?;

        console_log!("Session ended successfully: {}", session_id);
        
        Ok(())
    }

    /// End all sessions for a user
    pub async fn end_all_user_sessions(
        &self,
        user_id: &str,
    ) -> Result<u32, AppError> {
        let user_uuid = Uuid::parse_str(user_id)
            .map_err(|_| AppError::ValidationError("Invalid user ID format".to_string()))?;

        console_log!("Ending all sessions for user: {}", user_id);

        // Get all user sessions
        let sessions = self.session_repo.get_user_sessions(&user_uuid.to_string()).await?;

        let mut deleted_count = 0;
        for session in sessions {
            match self.session_repo.delete_session(&session.session_id().to_string()).await {
                Ok(_) => deleted_count += 1,
                Err(e) => {
                    warn!("Failed to delete session {}: {}", session.session_id(), e);
                }
            }
        }

        console_log!("Ended {} sessions for user: {}", deleted_count, user_id);
        
        Ok(deleted_count)
    }

    /// Update session security info (IP, user agent)
    pub async fn update_session_security(
        &self,
        session_id: &str,
        new_user_agent: Option<String>,
        new_ip_address: Option<IpAddr>,
    ) -> Result<UserSession, AppError> {
        let session_uuid = Uuid::parse_str(session_id)
            .map_err(|_| AppError::ValidationError("Invalid session ID format".to_string()))?;

        // Get current session
        let mut session = self.session_repo.get_session(&session_uuid.to_string()).await?
            .ok_or_else(|| AppError::AuthenticationError("Session not found".to_string()))?;

        // Update security info
        if let Some(user_agent) = new_user_agent {
            session.update_user_agent(Some(user_agent));
        }

        if let Some(ip) = new_ip_address {
            session.update_ip_address(Some(ip));
        }

        // Update in repository
        self.session_repo.update_session(&session_uuid.to_string(), session.clone()).await?;

        console_log!("Session security info updated: {}", session_id);
        
        Ok(session)
    }

    /// Get user sessions
    pub async fn get_user_sessions(
        &self,
        user_id: &str,
    ) -> Result<Vec<UserSession>, AppError> {
        let user_uuid = Uuid::parse_str(user_id)
            .map_err(|_| AppError::ValidationError("Invalid user ID format".to_string()))?;

        let sessions = self.session_repo.get_user_sessions(&user_uuid.to_string()).await?;
        
        console_log!("Retrieved {} sessions for user: {}", sessions.len(), user_id);
        
        Ok(sessions)
    }

    /// Get active session count for user
    pub async fn get_active_session_count(
        &self,
        user_id: &str,
    ) -> Result<usize, AppError> {
        let sessions = self.get_user_sessions(user_id).await?;
        let active_count = sessions.iter().filter(|s| s.is_valid()).count();
        
        Ok(active_count)
    }

    /// Cleanup expired sessions
    pub async fn cleanup_expired_sessions(&self) -> Result<u32, AppError> {
        console_log!("Starting expired session cleanup");
        
        let deleted_count = self.session_repo.cleanup_expired_sessions().await?;
        
        if deleted_count > 0 {
            console_log!("Cleaned up {} expired sessions", deleted_count);
        }
        
        Ok(deleted_count)
    }

    /// Associate token with session
    pub async fn associate_token_with_session(
        &self,
        session_id: &str,
        token_id: Uuid,
    ) -> Result<(), AppError> {
        let session_uuid = Uuid::parse_str(session_id)
            .map_err(|_| AppError::ValidationError("Invalid session ID format".to_string()))?;

        // Get current session
        let mut session = self.session_repo.get_session(&session_uuid.to_string()).await?
            .ok_or_else(|| AppError::AuthenticationError("Session not found".to_string()))?;

        // Associate token
        session.set_token(token_id);

        // Update in repository
        self.session_repo.update_session(&session_uuid.to_string(), session).await?;

        console_log!("Associated token {} with session {}", token_id, session_id);
        
        Ok(())
    }

    /// Check if user has active sessions
    pub async fn has_active_sessions(&self, user_id: &str) -> Result<bool, AppError> {
        let count = self.get_active_session_count(user_id).await?;
        Ok(count > 0)
    }

    /// Get session statistics
    pub async fn get_session_stats(&self) -> Result<SessionStats, AppError> {
        // This would typically query the database for stats
        // For now, we'll return basic stats
        Ok(SessionStats {
            total_sessions: 0,
            active_sessions: 0,
            expired_sessions: 0,
            average_session_duration_hours: 24.0,
        })
    }

    /// Set custom session duration
    pub fn set_session_duration(&mut self, duration: chrono::Duration) {
        self.session_duration = duration;
    }

    /// Get current session duration
    pub fn get_session_duration(&self) -> chrono::Duration {
        self.session_duration
    }

}

/// Session statistics
#[derive(Debug, serde::Serialize)]
pub struct SessionStats {
    pub total_sessions: u64,
    pub active_sessions: u64,
    pub expired_sessions: u64,
    pub average_session_duration_hours: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    // Tests for this service require a Database to construct repositories.
    // Omitted here due to environment constraints.
}