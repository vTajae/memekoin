//! Account Sessions Entity - Enhanced with business logic following schema v11

use serde::{Deserialize, Serialize};
use tracing::{info, warn, error};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AccountSession {
    pub(crate) id: String,
    pub(crate) data: String,
    pub(crate) expires: i64,
    pub(crate) created_at: chrono::DateTime<chrono::Utc>,
}

impl AccountSession {
    /// Create new account session
    pub(crate) fn new(
        id: String,
        data: String,
        expires: i64,
    ) -> Self {
        info!("Creating new account session with id: {}", id);
        
        Self {
            id,
            data,
            expires,
            created_at: chrono::Utc::now(),
        }
    }

    /// Create session with TTL (time to live in seconds)
    pub(crate) fn with_ttl(
        id: String,
        data: String,
        ttl_seconds: i64,
    ) -> Self {
        let expires = chrono::Utc::now().timestamp() + ttl_seconds;
        Self::new(id, data, expires)
    }

    /// Check if session is expired
    pub(crate) fn is_expired(&self) -> bool {
        let now = chrono::Utc::now().timestamp();
        now > self.expires
    }

    /// Check if session is valid (not expired)
    pub(crate) fn is_valid(&self) -> bool {
        !self.is_expired()
    }

    /// Get remaining TTL in seconds
    pub(crate) fn remaining_ttl(&self) -> i64 {
        let now = chrono::Utc::now().timestamp();
        std::cmp::max(0, self.expires - now)
    }

    /// Extend session expiry
    pub(crate) fn extend_expiry(&mut self, additional_seconds: i64) -> Result<(), String> {
        if self.is_expired() {
            warn!("Attempting to extend expired session: {}", self.id);
            return Err("Cannot extend expired session".to_string());
        }
        
        self.expires += additional_seconds;
        info!("Extended session {} by {} seconds", self.id, additional_seconds);
        Ok(())
    }

    /// Update session data
    pub(crate) fn update_data(&mut self, new_data: String) -> Result<(), String> {
        if self.is_expired() {
            warn!("Attempting to update expired session: {}", self.id);
            return Err("Cannot update expired session".to_string());
        }
        
        self.data = new_data;
        info!("Updated data for session: {}", self.id);
        Ok(())
    }

    /// Refresh session (extend expiry and optionally update data)
    pub(crate) fn refresh(&mut self, ttl_seconds: i64, new_data: Option<String>) -> Result<(), String> {
        if self.is_expired() {
            return Err("Cannot refresh expired session".to_string());
        }
        
        self.expires = chrono::Utc::now().timestamp() + ttl_seconds;
        
        if let Some(data) = new_data {
            self.data = data;
        }
        
        info!("Refreshed session: {}", self.id);
        Ok(())
    }

    /// Get expiry as DateTime
    pub(crate) fn expires_at(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::DateTime::from_timestamp(self.expires, 0)
            .unwrap_or(chrono::Utc::now())
    }

    /// Get session age in seconds
    pub(crate) fn age_seconds(&self) -> i64 {
        chrono::Utc::now().timestamp() - self.created_at.timestamp()
    }

    // Public accessors
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn data(&self) -> &str {
        &self.data
    }

    pub fn expires(&self) -> i64 {
        self.expires
    }

    pub fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.created_at
    }
}
