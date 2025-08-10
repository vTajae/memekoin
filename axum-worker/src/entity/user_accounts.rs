//! User Accounts Entity - Enhanced with business logic following schema v11
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use tracing::{info, warn, error};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserAccount {
    pub(crate) id: Uuid,
    pub(crate) user_id: Uuid,
    pub(crate) provider_id: i16,
    pub(crate) provider_user_id: String,
    pub(crate) is_active: bool,
    pub(crate) connected_at: chrono::DateTime<chrono::Utc>,
    pub(crate) last_login_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountStatus {
    Active,
    Inactive,
    Suspended,
    PendingVerification,
    Deactivated,
}

impl AccountStatus {
    pub fn as_str(&self) -> &str {
        match self {
            AccountStatus::Active => "active",
            AccountStatus::Inactive => "inactive",
            AccountStatus::Suspended => "suspended",
            AccountStatus::PendingVerification => "pending_verification",
            AccountStatus::Deactivated => "deactivated",
        }
    }

    pub fn can_login(&self) -> bool {
        matches!(self, AccountStatus::Active)
    }

    pub fn requires_action(&self) -> bool {
        matches!(self, AccountStatus::PendingVerification | AccountStatus::Suspended)
    }
}

impl UserAccount {
    /// Create new user account
    pub(crate) fn new(
        id: Uuid,
        user_id: Uuid,
        provider_id: i16,
        provider_user_id: String,
    ) -> Self {
        info!("Creating new user account for user_id: {} with provider_id: {}", user_id, provider_id);
        
        let now = chrono::Utc::now();
        Self {
            id,
            user_id,
            provider_id,
            provider_user_id,
            is_active: true,
            connected_at: now,
            last_login_at: None,
        }
    }

    /// Create inactive user account (requires activation)
    pub(crate) fn new_inactive(
        id: Uuid,
        user_id: Uuid,
        provider_id: i16,
        provider_user_id: String,
    ) -> Self {
        info!("Creating inactive user account for user_id: {} with provider_id: {}", user_id, provider_id);
        
        let now = chrono::Utc::now();
        Self {
            id,
            user_id,
            provider_id,
            provider_user_id,
            is_active: false,
            connected_at: now,
            last_login_at: None,
        }
    }

    /// Activate user account
    pub(crate) fn activate(&mut self) {
        if !self.is_active {
            self.is_active = true;
            info!("Activated user account: {}", self.id);
        }
    }

    /// Deactivate user account
    pub(crate) fn deactivate(&mut self) {
        if self.is_active {
            self.is_active = false;
            warn!("Deactivated user account: {}", self.id);
        }
    }

    /// Record successful login
    pub(crate) fn record_login(&mut self) -> Result<(), String> {
        if !self.is_active {
            error!("Login attempt on inactive account: {}", self.id);
            return Err("Account is not active".to_string());
        }
        
        self.last_login_at = Some(chrono::Utc::now());
        info!("Recorded login for user account: {}", self.id);
        Ok(())
    }

    /// Check if account can login
    pub(crate) fn can_login(&self) -> bool {
        self.is_active
    }

    /// Get account status
    pub(crate) fn get_status(&self) -> AccountStatus {
        if self.is_active {
            AccountStatus::Active
        } else {
            AccountStatus::Inactive
        }
    }

    /// Check if account was recently active (logged in within last 30 days)
    pub(crate) fn is_recently_active(&self) -> bool {
        match self.last_login_at {
            Some(last_login) => {
                let thirty_days_ago = chrono::Utc::now() - chrono::Duration::days(30);
                last_login > thirty_days_ago
            }
            None => false,
        }
    }

    /// Check if this is the user's first login
    pub(crate) fn is_first_login(&self) -> bool {
        self.last_login_at.is_none()
    }

    /// Get days since last login
    pub(crate) fn days_since_last_login(&self) -> Option<i64> {
        self.last_login_at.map(|last_login| {
            let duration = chrono::Utc::now() - last_login;
            duration.num_days()
        })
    }

    /// Get account age in days
    pub(crate) fn account_age_days(&self) -> i64 {
        let duration = chrono::Utc::now() - self.connected_at;
        duration.num_days()
    }

    /// Check if account is dormant (no login in 90+ days)
    pub(crate) fn is_dormant(&self) -> bool {
        match self.last_login_at {
            Some(last_login) => {
                let ninety_days_ago = chrono::Utc::now() - chrono::Duration::days(90);
                last_login < ninety_days_ago
            }
            None => {
                // Never logged in and created more than 90 days ago
                let ninety_days_ago = chrono::Utc::now() - chrono::Duration::days(90);
                self.connected_at < ninety_days_ago
            }
        }
    }

    /// Check if this is a local provider account
    pub(crate) fn is_local_account(&self) -> bool {
        self.provider_id == 1 // Assuming provider_id 1 is local
    }

    /// Check if this is an OAuth account
    pub(crate) fn is_oauth_account(&self) -> bool {
        !self.is_local_account()
    }

    /// Update provider user ID (for OAuth account linking updates)
    pub(crate) fn update_provider_user_id(&mut self, new_provider_user_id: String) {
        if self.provider_user_id != new_provider_user_id {
            info!("Updating provider user ID for account {}: {} -> {}", 
                  self.id, self.provider_user_id, new_provider_user_id);
            self.provider_user_id = new_provider_user_id;
        }
    }

    /// Get login frequency (logins per month)
    pub(crate) fn login_frequency(&self) -> f64 {
        let account_age_months = self.account_age_days() as f64 / 30.0;
        if account_age_months < 1.0 {
            match self.last_login_at {
                Some(_) => 1.0, // At least one login this month
                None => 0.0,
            }
        } else {
            // Rough estimate based on whether they've logged in
            match self.last_login_at {
                Some(_) => 1.0 / account_age_months,
                None => 0.0,
            }
        }
    }

    /// Get public account data (safe for API responses)
    pub(crate) fn to_public(&self) -> Self {
        Self {
            id: self.id,
            user_id: self.user_id,
            provider_id: self.provider_id,
            provider_user_id: self.provider_user_id.clone(),
            is_active: self.is_active,
            connected_at: self.connected_at,
            last_login_at: self.last_login_at,
        }
    }

    // Public accessors
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn provider_id(&self) -> i16 {
        self.provider_id
    }

    pub fn provider_user_id(&self) -> &str {
        &self.provider_user_id
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn connected_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.connected_at
    }

    pub fn last_login_at(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.last_login_at
    }

}
