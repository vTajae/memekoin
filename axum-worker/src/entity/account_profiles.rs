//! Account Profiles Entity - Enhanced with business logic following schema v11
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AccountProfile {
    pub(crate) id: Uuid,
    pub(crate) user_account_id: Uuid,
    pub(crate) email: Option<String>,
    pub(crate) username: Option<String>,
    pub(crate) display_name: Option<String>,
    pub(crate) avatar_url: Option<String>,
    pub(crate) profile_data: Option<JsonValue>,
    pub(crate) created_at: chrono::DateTime<chrono::Utc>,
    pub(crate) updated_at: chrono::DateTime<chrono::Utc>,
}

impl AccountProfile {
    /// Create new account profile
    pub(crate) fn new(
        id: Uuid,
        user_account_id: Uuid,
        email: Option<String>,
        username: Option<String>,
        display_name: Option<String>,
        avatar_url: Option<String>,
        profile_data: Option<JsonValue>,
    ) -> Self {
        info!("Creating new account profile for user_account_id: {}", user_account_id);
        
        let now = chrono::Utc::now();
        Self {
            id,
            user_account_id,
            email,
            username,
            display_name,
            avatar_url,
            profile_data,
            created_at: now,
            updated_at: now,
        }
    }

    /// Update profile display name
    pub(crate) fn update_display_name(&mut self, new_display_name: String) {
        self.display_name = Some(new_display_name.clone());
        self.updated_at = chrono::Utc::now();
        info!("Display name updated for account profile: {}", self.id);
    }

    /// Update profile email
    pub(crate) fn update_email(&mut self, new_email: String) {
        self.email = Some(new_email.clone());
        self.updated_at = chrono::Utc::now();
        info!("Email updated for account profile: {}", self.id);
    }

    /// Update profile username
    pub(crate) fn update_username(&mut self, new_username: String) -> Result<(), String> {
        if new_username.len() < 3 {
            return Err("Username must be at least 3 characters long".to_string());
        }
        if new_username.len() > 50 {
            return Err("Username must be less than 50 characters long".to_string());
        }
        
        self.username = Some(new_username.clone());
        self.updated_at = chrono::Utc::now();
        info!("Username updated for account profile: {}", self.id);
        Ok(())
    }

    /// Update avatar URL
    pub(crate) fn update_avatar_url(&mut self, new_avatar_url: Option<String>) {
        self.avatar_url = new_avatar_url;
        self.updated_at = chrono::Utc::now();
        info!("Avatar URL updated for account profile: {}", self.id);
    }

    /// Update profile data (JSON)
    pub(crate) fn update_profile_data(&mut self, new_profile_data: Option<JsonValue>) {
        self.profile_data = new_profile_data;
        self.updated_at = chrono::Utc::now();
        info!("Profile data updated for account profile: {}", self.id);
    }

    /// Get effective display name (display_name > username > email > id)
    pub(crate) fn effective_display_name(&self) -> String {
        if let Some(display_name) = &self.display_name {
            display_name.clone()
        } else if let Some(username) = &self.username {
            username.clone()
        } else if let Some(email) = &self.email {
            email.clone()
        } else {
            format!("User-{}", &self.id.to_string()[..8])
        }
    }

    /// Check if profile has complete information
    pub(crate) fn is_complete(&self) -> bool {
        self.display_name.is_some() && self.email.is_some()
    }

    /// Get public profile data (safe for API responses)
    pub(crate) fn to_public(&self) -> Self {
        Self {
            id: self.id,
            user_account_id: self.user_account_id,
            email: None, // Don't expose email in public profile
            username: self.username.clone(),
            display_name: self.display_name.clone(),
            avatar_url: self.avatar_url.clone(),
            profile_data: None, // Don't expose raw profile data
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

    pub fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }

    pub fn username(&self) -> Option<&str> {
        self.username.as_deref()
    }

    pub fn display_name(&self) -> Option<&str> {
        self.display_name.as_deref()
    }

    pub fn avatar_url(&self) -> Option<&str> {
        self.avatar_url.as_deref()
    }

    pub fn profile_data(&self) -> Option<&JsonValue> {
        self.profile_data.as_ref()
    }

    pub fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.updated_at
    }
}
