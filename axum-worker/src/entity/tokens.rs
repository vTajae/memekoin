//! Tokens Entity - Enhanced with business logic following schema v11
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tracing::{info, warn};
use rand::{distributions::Alphanumeric, Rng};
use sha2::{Sha256, Digest};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Token {
    pub(crate) id: Uuid,
    pub(crate) user_account_id: Uuid,
    pub(crate) type_id: i16,
    pub(crate) value: String, // Store hashed value
    pub(crate) expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub(crate) last_used_at: Option<chrono::DateTime<chrono::Utc>>,
    pub(crate) created_at: chrono::DateTime<chrono::Utc>,
    pub(crate) updated_at: chrono::DateTime<chrono::Utc>,
}

impl Token {
    /// Generate a secure random token
    pub(crate) fn generate_secure_token(length: usize) -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect()
    }

    /// Hash a token value for storage
    pub(crate) fn hash_token(token: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Create new token with auto-generated value
    pub(crate) fn new(
        id: Uuid,
        user_account_id: Uuid,
        type_id: i16,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> (Self, String) {
        let raw_token = Self::generate_secure_token(64);
        let hashed_value = Self::hash_token(&raw_token);
        
        info!("Creating new token for user_account_id: {} with type_id: {}", user_account_id, type_id);
        
        let now = chrono::Utc::now();
        let token = Self {
            id,
            user_account_id,
            type_id,
            value: hashed_value,
            expires_at,
            last_used_at: None,
            created_at: now,
            updated_at: now,
        };
        
        (token, raw_token)
    }

    /// Create token with custom value (for testing or special cases)
    pub(crate) fn with_value(
        id: Uuid,
        user_account_id: Uuid,
        type_id: i16,
        raw_value: String,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Self {
        let hashed_value = Self::hash_token(&raw_value);
        
        info!("Creating token with custom value for user_account_id: {}", user_account_id);
        
        let now = chrono::Utc::now();
        Self {
            id,
            user_account_id,
            type_id,
            value: hashed_value,
            expires_at,
            last_used_at: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Verify a raw token against this stored token
    pub(crate) fn verify(&self, raw_token: &str) -> bool {
        let hashed_input = Self::hash_token(raw_token);
        self.value == hashed_input
    }

    /// Check if token is expired
    pub(crate) fn is_expired(&self) -> bool {
        match self.expires_at {
            Some(expiry) => chrono::Utc::now() > expiry,
            None => false, // Never expires
        }
    }

    /// Check if token is valid (not expired and not revoked)
    pub(crate) fn is_valid(&self) -> bool {
        !self.is_expired()
    }

    /// Mark token as used (update last_used_at)
    pub(crate) fn mark_as_used(&mut self) {
        self.last_used_at = Some(chrono::Utc::now());
        self.updated_at = chrono::Utc::now();
        info!("Token {} marked as used", self.id);
    }

    /// Extend token expiry
    pub(crate) fn extend_expiry(&mut self, additional_seconds: i64) -> Result<(), String> {
        if self.is_expired() {
            warn!("Attempting to extend expired token: {}", self.id);
            return Err("Cannot extend expired token".to_string());
        }
        
        match self.expires_at {
            Some(current_expiry) => {
                let new_expiry = current_expiry + chrono::Duration::seconds(additional_seconds);
                self.expires_at = Some(new_expiry);
                self.updated_at = chrono::Utc::now();
                info!("Extended token {} expiry by {} seconds", self.id, additional_seconds);
                Ok(())
            }
            None => {
                warn!("Attempting to extend never-expiring token: {}", self.id);
                Err("Cannot extend never-expiring token".to_string())
            }
        }
    }

    /// Revoke token (set expiry to now)
    pub(crate) fn revoke(&mut self) {
        self.expires_at = Some(chrono::Utc::now());
        self.updated_at = chrono::Utc::now();
        info!("Revoked token: {}", self.id);
    }

    /// Check if token has been used recently (within last hour)
    pub(crate) fn is_recently_used(&self) -> bool {
        match self.last_used_at {
            Some(last_used) => {
                let one_hour_ago = chrono::Utc::now() - chrono::Duration::hours(1);
                last_used > one_hour_ago
            }
            None => false,
        }
    }

    /// Get remaining time to live in seconds
    pub(crate) fn remaining_ttl(&self) -> Option<i64> {
        self.expires_at.map(|expiry| {
            let remaining = expiry.timestamp() - chrono::Utc::now().timestamp();
            std::cmp::max(0, remaining)
        })
    }

    /// Get token age in seconds
    pub(crate) fn age_seconds(&self) -> i64 {
        chrono::Utc::now().timestamp() - self.created_at.timestamp()
    }

    /// Check if token is long-lived (expires in more than 24 hours or never)
    pub(crate) fn is_long_lived(&self) -> bool {
        match self.expires_at {
            Some(expiry) => {
                let twenty_four_hours = chrono::Utc::now() + chrono::Duration::hours(24);
                expiry > twenty_four_hours
            }
            None => true, // Never expires = long-lived
        }
    }

    /// Get usage frequency (times used per day since creation)
    pub(crate) fn usage_frequency(&self) -> f64 {
        let age_days = self.age_seconds() as f64 / 86400.0; // seconds to days
        if age_days < 1.0 {
            match self.last_used_at {
                Some(_) => 1.0, // Used at least once today
                None => 0.0,
            }
        } else {
            match self.last_used_at {
                Some(_) => 1.0 / age_days, // Rough estimate
                None => 0.0,
            }
        }
    }

    /// Get safe token info (without sensitive data)
    pub(crate) fn to_public(&self) -> Self {
        Self {
            id: self.id,
            user_account_id: self.user_account_id,
            type_id: self.type_id,
            value: "[hidden]".to_string(), // Never expose token value
            expires_at: self.expires_at,
            last_used_at: self.last_used_at,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }

    // Public accessors
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn user_account_id(&self) -> Uuid {
        self.user_account_id
    }

    pub fn type_id(&self) -> i16 {
        self.type_id
    }

    pub fn expires_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.expires_at
    }

    pub fn last_used_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.last_used_at
    }

    pub fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.updated_at
    }
}
