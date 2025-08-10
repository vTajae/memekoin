//! Audit Log Entity - Enhanced with business logic following schema v11
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use uuid::Uuid;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuditLog {
    pub(crate) id: Uuid,
    pub(crate) user_id: Option<Uuid>,
    pub(crate) event: String,
    pub(crate) ip_address: Option<String>,
    pub(crate) success: bool,
    pub(crate) created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditEvent {
    Login,
    LoginFailed,
    Logout,
    PasswordChange,
    PasswordChangeFailed,
    ProfileUpdate,
    AccountCreate,
    AccountDelete,
    TokenCreate,
    TokenRevoke,
    PermissionChange,
    DataAccess,
    DataModification,
    SecurityEvent,
    Custom(String),
}

impl AuditEvent {
    pub fn as_str(&self) -> &str {
        match self {
            AuditEvent::Login => "login",
            AuditEvent::LoginFailed => "login_failed",
            AuditEvent::Logout => "logout",
            AuditEvent::PasswordChange => "password_change",
            AuditEvent::PasswordChangeFailed => "password_change_failed",
            AuditEvent::ProfileUpdate => "profile_update",
            AuditEvent::AccountCreate => "account_create",
            AuditEvent::AccountDelete => "account_delete",
            AuditEvent::TokenCreate => "token_create",
            AuditEvent::TokenRevoke => "token_revoke",
            AuditEvent::PermissionChange => "permission_change",
            AuditEvent::DataAccess => "data_access",
            AuditEvent::DataModification => "data_modification",
            AuditEvent::SecurityEvent => "security_event",
            AuditEvent::Custom(event) => event,
        }
    }
}

impl From<AuditEvent> for String {
    fn from(event: AuditEvent) -> Self {
        event.as_str().to_string()
    }
}

impl AuditLog {
    /// Create new audit log entry
    pub(crate) fn new(
        id: Uuid,
        user_id: Option<Uuid>,
        event: AuditEvent,
        ip_address: Option<IpAddr>,
        success: bool,
    ) -> Self {
        let event_str = String::from(event);
        let ip_str = ip_address.map(|ip| ip.to_string());
        
        info!("Creating audit log entry: {} for user {:?}", event_str, user_id);
        
        Self {
            id,
            user_id,
            event: event_str,
            ip_address: ip_str,
            success,
            created_at: chrono::Utc::now(),
        }
    }

    /// Create success audit log
    pub(crate) fn success(
        user_id: Option<Uuid>,
        event: AuditEvent,
        ip_address: Option<IpAddr>,
    ) -> Self {
        Self::new(Uuid::new_v4(), user_id, event, ip_address, true)
    }

    /// Create failure audit log
    pub(crate) fn failure(
        user_id: Option<Uuid>,
        event: AuditEvent,
        ip_address: Option<IpAddr>,
    ) -> Self {
        Self::new(Uuid::new_v4(), user_id, event, ip_address, false)
    }

    /// Create anonymous audit log (no user_id)
    pub(crate) fn anonymous(
        event: AuditEvent,
        ip_address: Option<IpAddr>,
        success: bool,
    ) -> Self {
        Self::new(Uuid::new_v4(), None, event, ip_address, success)
    }

    /// Check if this is a security-relevant event
    pub(crate) fn is_security_event(&self) -> bool {
        matches!(
            self.event.as_str(),
            "login_failed" | "password_change_failed" | "security_event" | "permission_change"
        ) || (!self.success && matches!(
            self.event.as_str(),
            "login" | "password_change" | "account_create"
        ))
    }

    /// Check if this is a failed event
    pub(crate) fn is_failure(&self) -> bool {
        !self.success
    }

    /// Get event severity level
    pub(crate) fn severity_level(&self) -> &'static str {
        if self.is_security_event() {
            if self.is_failure() {
                "high"
            } else {
                "medium"
            }
        } else if self.is_failure() {
            "medium"
        } else {
            "low"
        }
    }

    /// Get log age in seconds
    pub(crate) fn age_seconds(&self) -> i64 {
        chrono::Utc::now().timestamp() - self.created_at.timestamp()
    }

    /// Check if log entry is recent (within last hour)
    pub(crate) fn is_recent(&self) -> bool {
        self.age_seconds() < 3600
    }

    /// Get sanitized version for public API (remove sensitive info)
    pub(crate) fn to_public(&self) -> Self {
        Self {
            id: self.id,
            user_id: self.user_id,
            event: self.event.clone(),
            ip_address: self.ip_address.as_ref().map(|_| "[hidden]".to_string()), // Hide actual IP
            success: self.success,
            created_at: self.created_at,
        }
    }

    // Public accessors
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn user_id(&self) -> Option<Uuid> {
        self.user_id
    }

    pub fn event(&self) -> &str {
        &self.event
    }

    pub fn ip_address(&self) -> Option<&str> {
        self.ip_address.as_deref()
    }

    pub fn is_success(&self) -> bool {
        self.success
    }

    pub fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.created_at
    }
}
