//! Users Entity - Enhanced with business logic following schema v11
#![allow(dead_code)]

use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
// use tokio_postgres_utils::FromRow; // Disabled for WASM compatibility
use uuid::Uuid;
use tracing::{info, error};

#[derive(Debug, Clone, Serialize, Deserialize)] // Removed FromRow for WASM compatibility
pub struct User {
    pub(crate) id: Uuid,
    pub(crate) email: String,
    pub(crate) first_name: Option<String>,
    pub(crate) last_name: Option<String>,
    pub(crate) password: Option<String>,
    pub(crate) created_at: chrono::DateTime<chrono::Utc>,
}

impl User {
    /// Create new user with password hashing
    pub(crate) fn new(
        id: Uuid,
        email: String,
        first_name: Option<String>,
        last_name: Option<String>,
        password: Option<String>,
        _is_oauth: bool,
    ) -> Self {
        let hashed_password = password.map(|pwd| {
            hash(pwd, DEFAULT_COST).unwrap_or_else(|e| {
                error!("Failed to hash password: {}", e);
                "$2b$12$invalid_hash".to_string()
            })
        });

        info!("Creating new user with email: {}", email);
        
        Self {
            id,
            email,
            first_name,
            last_name,
            password: hashed_password,
            
            created_at: chrono::Utc::now(),
        }
    }

    /// Convert from repository User to entity User
    pub fn from_repository_user(repo_user: crate::repository::user::User) -> Self {
        Self {
            id: repo_user.id,
            email: repo_user.email,
            first_name: repo_user.first_name,
            last_name: repo_user.last_name,
            password: repo_user.password, // Already hashed in repository
            created_at: repo_user.created_at,
        }
    }

    /// Verify password against stored hash
    pub(crate) fn verify_password(&self, password: &str) -> bool {
        match &self.password {
            Some(hash) => verify(password, hash).unwrap_or(false),
            None => {
                info!("Password verification attempted on user without password: {}", self.email);
                false
            }
        }
    }

    /// Update password with new hash
    pub(crate) fn update_password(&mut self, new_password: String) -> Result<(), String> {
        match hash(new_password, DEFAULT_COST) {
            Ok(hashed) => {
                self.password = Some(hashed);
                info!("Password updated for user: {}", self.email);
                Ok(())
            }
            Err(e) => {
                error!("Failed to hash new password for user {}: {}", self.email, e);
                Err("Failed to update password".to_string())
            }
        }
    }

    /// Check if user can login with password
    pub(crate) fn can_password_login(&self) -> bool {
        // If the password is null they must be an OAuth user
        self.password.is_some()
    }


    /// Get user's display name (first name + last name or email)
    pub(crate) fn display_name(&self) -> String {
        match (&self.first_name, &self.last_name) {
            (Some(first), Some(last)) => format!("{} {}", first, last),
            (Some(first), None) => first.clone(),
            (None, Some(last)) => last.clone(),
            (None, None) => self.email.clone(),
        }
    }

    /// Get public user data (without password hash)
    pub(crate) fn to_public(&self) -> Self {
        Self {
            id: self.id,
            email: self.email.clone(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            password: None, // Never expose password hash
            created_at: self.created_at,
        }
    }

    // Public accessors
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn first_name(&self) -> Option<&str> {
        self.first_name.as_deref()
    }

    pub fn last_name(&self) -> Option<&str> {
        self.last_name.as_deref()
    }

    pub fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.created_at
    }
}
