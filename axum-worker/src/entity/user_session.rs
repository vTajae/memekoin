//! Minimal UserSession entity for compatibility
//! Note: Sessions are primarily managed by tower-session, this is for legacy compatibility

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use std::net::IpAddr;

/// Minimal user session entity for backwards compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    pub id: String,
    pub user_id: Uuid,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
    pub token_id: Option<Uuid>,
}

impl UserSession {
    /// Create new user session
    pub fn new(
        id: String,
        user_id: Uuid,
        expires_at: DateTime<Utc>,
        user_agent: Option<String>,
        ip_address: Option<String>,
    ) -> Self {
        Self {
            id,
            user_id,
            expires_at,
            created_at: Utc::now(),
            user_agent,
            ip_address,
            token_id: None,
        }
    }

    /// Create session with default duration (24 hours)
    pub fn with_default_duration(
        id: String,
        user_id: Uuid,
        user_agent: Option<String>,
        ip_address: Option<IpAddr>,
    ) -> Self {
        let expires_at = Utc::now() + Duration::hours(24);
        let ip_str = ip_address.map(|ip| ip.to_string());
        Self::new(id, user_id, expires_at, user_agent, ip_str)
    }

    /// Get session ID
    pub fn session_id(&self) -> &str {
        &self.id
    }

    /// Check if session is expired
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    /// Get user ID
    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    /// Get expiration time
    pub fn expires_at(&self) -> DateTime<Utc> {
        self.expires_at
    }

    /// Get creation time
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    /// Check if session is valid (not expired)
    pub fn is_valid(&self) -> bool {
        !self.is_expired()
    }

    /// Refresh session expiration
    pub fn refresh(&mut self, duration: Duration) -> Result<(), String> {
        self.expires_at = Utc::now() + duration;
        Ok(())
    }

    /// Update user agent
    pub fn update_user_agent(&mut self, user_agent: Option<String>) {
        self.user_agent = user_agent;
    }

    /// Update IP address
    pub fn update_ip_address(&mut self, ip_address: Option<IpAddr>) {
        self.ip_address = ip_address.map(|ip| ip.to_string());
    }

    /// Set associated token ID
    pub fn set_token(&mut self, token_id: Uuid) {
        self.token_id = Some(token_id);
    }

    /// Get associated token ID
    pub fn token_id(&self) -> Option<Uuid> {
        self.token_id
    }

    /// Check if session has an associated token
    pub fn has_token(&self) -> bool {
        self.token_id.is_some()
    }

    /// Get user agent
    pub fn user_agent(&self) -> Option<&str> {
        self.user_agent.as_deref()
    }

    /// Get IP address
    pub fn ip_address(&self) -> Option<&str> {
        self.ip_address.as_deref()
    }
}

/// Session status enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionStatus {
    Active,
    Expired,
    Revoked,
}