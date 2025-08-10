//! Token Scopes Entity - Enhanced with business logic following schema v11
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TokenScope {
    pub(crate) token_id: Uuid,
    pub(crate) scope: String,
    pub(crate) created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Scope {
    // Read permissions
    ReadProfile,
    ReadEmail,
    ReadUserData,
    ReadAuditLog,
    
    // Write permissions
    WriteProfile,
    WriteUserData,
    
    // Admin permissions
    AdminUsers,
    AdminSystem,
    AdminAudit,
    
    // API permissions
    ApiAccess,
    ApiWrite,
    ApiAdmin,
    
    // Special permissions
    ResetPassword,
    VerifyEmail,
    InviteUsers,
    
    // Custom scopes
    Custom(String),
}

impl Scope {
    pub fn as_str(&self) -> &str {
        match self {
            Scope::ReadProfile => "read:profile",
            Scope::ReadEmail => "read:email",
            Scope::ReadUserData => "read:user_data",
            Scope::ReadAuditLog => "read:audit_log",
            Scope::WriteProfile => "write:profile",
            Scope::WriteUserData => "write:user_data",
            Scope::AdminUsers => "admin:users",
            Scope::AdminSystem => "admin:system",
            Scope::AdminAudit => "admin:audit",
            Scope::ApiAccess => "api:access",
            Scope::ApiWrite => "api:write",
            Scope::ApiAdmin => "api:admin",
            Scope::ResetPassword => "reset:password",
            Scope::VerifyEmail => "verify:email",
            Scope::InviteUsers => "invite:users",
            Scope::Custom(s) => s,
        }
    }

    pub fn description(&self) -> &str {
        match self {
            Scope::ReadProfile => "Read user profile information",
            Scope::ReadEmail => "Read user email address",
            Scope::ReadUserData => "Read user data and preferences",
            Scope::ReadAuditLog => "Read audit log entries",
            Scope::WriteProfile => "Modify user profile information",
            Scope::WriteUserData => "Modify user data and preferences",
            Scope::AdminUsers => "Administrate user accounts",
            Scope::AdminSystem => "System administration access",
            Scope::AdminAudit => "Audit log administration",
            Scope::ApiAccess => "Basic API access",
            Scope::ApiWrite => "API write permissions",
            Scope::ApiAdmin => "API administration permissions",
            Scope::ResetPassword => "Reset user passwords",
            Scope::VerifyEmail => "Verify email addresses",
            Scope::InviteUsers => "Send user invitations",
            Scope::Custom(_) => "Custom permission scope",
        }
    }

    pub fn is_admin(&self) -> bool {
        matches!(self, Scope::AdminUsers | Scope::AdminSystem | Scope::AdminAudit | Scope::ApiAdmin)
    }

    pub fn is_write(&self) -> bool {
        matches!(self, Scope::WriteProfile | Scope::WriteUserData | Scope::ApiWrite) || self.is_admin()
    }

    pub fn is_read(&self) -> bool {
        matches!(self, Scope::ReadProfile | Scope::ReadEmail | Scope::ReadUserData | Scope::ReadAuditLog)
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "read:profile" => Scope::ReadProfile,
            "read:email" => Scope::ReadEmail,
            "read:user_data" => Scope::ReadUserData,
            "read:audit_log" => Scope::ReadAuditLog,
            "write:profile" => Scope::WriteProfile,
            "write:user_data" => Scope::WriteUserData,
            "admin:users" => Scope::AdminUsers,
            "admin:system" => Scope::AdminSystem,
            "admin:audit" => Scope::AdminAudit,
            "api:access" => Scope::ApiAccess,
            "api:write" => Scope::ApiWrite,
            "api:admin" => Scope::ApiAdmin,
            "reset:password" => Scope::ResetPassword,
            "verify:email" => Scope::VerifyEmail,
            "invite:users" => Scope::InviteUsers,
            _ => Scope::Custom(s.to_string()),
        }
    }
}

impl From<Scope> for String {
    fn from(scope: Scope) -> Self {
        scope.as_str().to_string()
    }
}

impl TokenScope {
    /// Create new token scope
    pub(crate) fn new(token_id: Uuid, scope: Scope) -> Self {
        let scope_str = String::from(scope);
        info!("Adding scope '{}' to token: {}", scope_str, token_id);
        
        Self {
            token_id,
            scope: scope_str,
            created_at: chrono::Utc::now(),
        }
    }

    /// Create multiple token scopes
    pub(crate) fn create_multiple(token_id: Uuid, scopes: Vec<Scope>) -> Vec<Self> {
        info!("Adding {} scopes to token: {}", scopes.len(), token_id);
        scopes.into_iter()
            .map(|scope| Self::new(token_id, scope))
            .collect()
    }

    /// Get scope as enum
    pub(crate) fn get_scope(&self) -> Scope {
        Scope::from_str(&self.scope)
    }

    /// Check if this is an admin scope
    pub(crate) fn is_admin_scope(&self) -> bool {
        self.get_scope().is_admin()
    }

    /// Check if this is a write scope
    pub(crate) fn is_write_scope(&self) -> bool {
        self.get_scope().is_write()
    }

    /// Check if this is a read scope
    pub(crate) fn is_read_scope(&self) -> bool {
        self.get_scope().is_read()
    }

    /// Get scope age in seconds
    pub(crate) fn age_seconds(&self) -> i64 {
        chrono::Utc::now().timestamp() - self.created_at.timestamp()
    }

    // Public accessors
    pub fn token_id(&self) -> Uuid {
        self.token_id
    }

    pub fn scope(&self) -> &str {
        &self.scope
    }

    pub fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.created_at
    }
}

/// Utility functions for working with multiple scopes
pub struct ScopeSet {
    scopes: HashSet<Scope>,
}

impl ScopeSet {
    pub fn new() -> Self {
        Self {
            scopes: HashSet::new(),
        }
    }

    pub fn from_token_scopes(token_scopes: Vec<TokenScope>) -> Self {
        let scopes = token_scopes
            .into_iter()
            .map(|ts| ts.get_scope())
            .collect();
        
        Self { scopes }
    }

    pub fn add(&mut self, scope: Scope) -> bool {
        self.scopes.insert(scope)
    }

    pub fn remove(&mut self, scope: &Scope) -> bool {
        self.scopes.remove(scope)
    }

    pub fn contains(&self, scope: &Scope) -> bool {
        self.scopes.contains(scope)
    }

    pub fn has_admin_access(&self) -> bool {
        self.scopes.iter().any(|s| s.is_admin())
    }

    pub fn has_write_access(&self) -> bool {
        self.scopes.iter().any(|s| s.is_write())
    }

    pub fn has_read_access(&self) -> bool {
        self.scopes.iter().any(|s| s.is_read())
    }

    pub fn can_access(&self, required_scope: &Scope) -> bool {
        self.contains(required_scope) || 
        (required_scope.is_read() && self.has_admin_access()) ||
        (required_scope.is_write() && self.has_admin_access())
    }

    pub fn to_strings(&self) -> Vec<String> {
        self.scopes.iter()
            .map(|s| s.as_str().to_string())
            .collect()
    }

    pub fn len(&self) -> usize {
        self.scopes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.scopes.is_empty()
    }
}
