//! Session Entity - Aligned with sessions_table schema for Phase 2 authentication

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tracing::{info, warn, error};
use std::net::IpAddr;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserSession {
    pub(crate) session_id: Uuid,
    pub(crate) user_id: Uuid,
    pub(crate) token_id: Option<Uuid>, // Reference to OAuth access token
    pub(crate) expires_at: chrono::DateTime<chrono::Utc>,
    pub(crate) created_at: chrono::DateTime<chrono::Utc>,
    pub(crate) user_agent: Option<String>,
    pub(crate) ip_address: Option<IpAddr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionStatus {
    Active,
    Expired,
    Revoked,
    Inactive,
}

impl SessionStatus {
    pub fn as_str(&self) -> &str {
        match self {
            SessionStatus::Active => "active",
            SessionStatus::Expired => "expired", 
            SessionStatus::Revoked => "revoked",
            SessionStatus::Inactive => "inactive",
        }
    }

    pub fn is_valid(&self) -> bool {
        matches!(self, SessionStatus::Active)
    }
}

impl UserSession {
    /// Create new user session
    pub(crate) fn new(
        session_id: Uuid,
        user_id: Uuid,
        token_id: Option<Uuid>,
        session_duration: chrono::Duration,
        user_agent: Option<String>,
        ip_address: Option<IpAddr>,
    ) -> Self {
        let now = chrono::Utc::now();
        let expires_at = now + session_duration;

        info!("Creating new user session for user_id: {} with session_id: {}", user_id, session_id);

        Self {
            session_id,
            user_id,
            token_id,
            expires_at,
            created_at: now,
            user_agent,
            ip_address,
        }
    }

    /// Create session with default duration (24 hours)
    pub(crate) fn with_default_duration(
        session_id: Uuid,
        user_id: Uuid,
        token_id: Option<Uuid>,
        user_agent: Option<String>,
        ip_address: Option<IpAddr>,
    ) -> Self {
        let default_duration = chrono::Duration::hours(24);
        Self::new(session_id, user_id, token_id, default_duration, user_agent, ip_address)
    }

    /// Get session ID
    pub fn id(&self) -> &Uuid {
        &self.session_id
    }

    /// Check if session is expired
    pub(crate) fn is_expired(&self) -> bool {
        chrono::Utc::now() > self.expires_at
    }

    /// Check if session is valid (not expired)
    pub(crate) fn is_valid(&self) -> bool {
        !self.is_expired()
    }

    /// Get session status
    pub(crate) fn get_status(&self) -> SessionStatus {
        if self.is_expired() {
            SessionStatus::Expired
        } else {
            SessionStatus::Active
        }
    }

    /// Extend session expiry
    pub(crate) fn extend_expiry(&mut self, additional_duration: chrono::Duration) -> Result<(), String> {
        if self.is_expired() {
            warn!("Attempting to extend expired session: {}", self.session_id);
            return Err("Cannot extend expired session".to_string());
        }

        self.expires_at = self.expires_at + additional_duration;
        info!("Extended session {} expiry by {} seconds", self.session_id, additional_duration.num_seconds());
        Ok(())
    }

    /// Refresh session (extend to new duration from now)
    pub(crate) fn refresh(&mut self, new_duration: chrono::Duration) -> Result<(), String> {
        if self.is_expired() {
            warn!("Attempting to refresh expired session: {}", self.session_id);
            return Err("Cannot refresh expired session".to_string());
        }

        self.expires_at = chrono::Utc::now() + new_duration;
        info!("Refreshed session: {} with new duration", self.session_id);
        Ok(())
    }

    /// Revoke session (set expiry to now)
    pub(crate) fn revoke(&mut self) {
        self.expires_at = chrono::Utc::now();
        info!("Revoked session: {}", self.session_id);
    }

    /// Associate token with session
    pub(crate) fn set_token(&mut self, token_id: Uuid) {
        self.token_id = Some(token_id);
        info!("Associated token {} with session {}", token_id, self.session_id);
    }

    /// Remove token association
    pub(crate) fn clear_token(&mut self) {
        if let Some(old_token) = self.token_id {
            info!("Cleared token {} from session {}", old_token, self.session_id);
        }
        self.token_id = None;
    }

    /// Get remaining time to live
    pub(crate) fn remaining_ttl(&self) -> chrono::Duration {
        if self.is_expired() {
            chrono::Duration::zero()
        } else {
            self.expires_at - chrono::Utc::now()
        }
    }

    /// Get remaining TTL in seconds
    pub(crate) fn remaining_ttl_seconds(&self) -> i64 {
        self.remaining_ttl().num_seconds()
    }

    /// Get session age
    pub(crate) fn age(&self) -> chrono::Duration {
        chrono::Utc::now() - self.created_at
    }

    /// Get session age in seconds
    pub(crate) fn age_seconds(&self) -> i64 {
        self.age().num_seconds()
    }

    /// Check if session was created recently (within last hour)
    pub(crate) fn is_recently_created(&self) -> bool {
        let one_hour_ago = chrono::Utc::now() - chrono::Duration::hours(1);
        self.created_at > one_hour_ago
    }

    /// Check if session is long-lived (expires in more than 12 hours)
    pub(crate) fn is_long_lived(&self) -> bool {
        let twelve_hours = chrono::Duration::hours(12);
        self.remaining_ttl() > twelve_hours
    }

    /// Check if session has associated token
    pub(crate) fn has_token(&self) -> bool {
        self.token_id.is_some()
    }

    /// Update user agent (for session hijacking detection)
    pub(crate) fn update_user_agent(&mut self, new_user_agent: Option<String>) -> Result<(), String> {
        if self.is_expired() {
            return Err("Cannot update expired session".to_string());
        }

        if self.user_agent != new_user_agent {
            warn!("User agent change detected for session {}: {:?} -> {:?}", 
                  self.session_id, self.user_agent, new_user_agent);
        }

        self.user_agent = new_user_agent;
        Ok(())
    }

    /// Update IP address (for session hijacking detection) 
    pub(crate) fn update_ip_address(&mut self, new_ip: Option<IpAddr>) -> Result<(), String> {
        if self.is_expired() {
            return Err("Cannot update expired session".to_string());
        }

        if self.ip_address != new_ip {
            warn!("IP address change detected for session {}: {:?} -> {:?}", 
                  self.session_id, self.ip_address, new_ip);
        }

        self.ip_address = new_ip;
        Ok(())
    }

    /// Get safe session data (without sensitive info)
    pub(crate) fn to_public(&self) -> Self {
        Self {
            session_id: self.session_id,
            user_id: self.user_id,
            token_id: None, // Don't expose token ID in public view
            expires_at: self.expires_at,
            created_at: self.created_at,
            user_agent: self.user_agent.clone(),
            ip_address: None, // Don't expose IP in public view
        }
    }

    // Public accessors
    pub fn session_id(&self) -> Uuid {
        self.session_id
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn token_id(&self) -> Option<Uuid> {
        self.token_id
    }

    pub fn expires_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.expires_at
    }

    pub fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.created_at
    }

    pub fn user_agent(&self) -> Option<&str> {
        self.user_agent.as_deref()
    }

    pub fn ip_address(&self) -> Option<IpAddr> {
        self.ip_address
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let session_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let duration = chrono::Duration::hours(1);
        
        let session = UserSession::new(
            session_id,
            user_id,
            None,
            duration,
            Some("test-agent".to_string()),
            Some("127.0.0.1".parse().unwrap()),
        );

        assert_eq!(session.session_id, session_id);
        assert_eq!(session.user_id, user_id);
        assert!(session.is_valid());
        assert!(!session.is_expired());
    }

    #[test]
    fn test_session_expiry() {
        let session_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let duration = chrono::Duration::seconds(-1); // Already expired
        
        let session = UserSession::new(
            session_id,
            user_id,
            None,
            duration,
            None,
            None,
        );

        assert!(!session.is_valid());
        assert!(session.is_expired());
        assert_eq!(session.get_status(), SessionStatus::Expired);
    }

    #[test] 
    fn test_session_refresh() {
        let session_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let duration = chrono::Duration::hours(1);
        
        let mut session = UserSession::new(
            session_id,
            user_id,
            None,
            duration,
            None,
            None,
        );

        let original_expires = session.expires_at;
        let result = session.refresh(chrono::Duration::hours(2));
        
        assert!(result.is_ok());
        assert!(session.expires_at > original_expires);
    }
}