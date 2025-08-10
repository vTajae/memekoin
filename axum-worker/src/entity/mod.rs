//! Entity module - Entities aligned with actual database schema
//! 
//! Uses tower-session for session management, OAuth flow with proper database schema
#![allow(dead_code)]

pub mod account_profiles;
pub mod audit_log;
pub mod providers;
pub mod token_scopes;
pub mod token_types;
pub mod tokens;
pub mod user_accounts;
pub mod user_session;
pub mod users;

// Re-export main entity types for convenience
pub use account_profiles::AccountProfile;
pub use audit_log::{AuditLog, AuditEvent};
pub use providers::{Provider, ProviderType};
pub use token_scopes::{TokenScope, Scope, ScopeSet};
pub use token_types::{TokenType, TokenTypeVariant, TokenExpiration};
pub use tokens::Token;
pub use user_accounts::{UserAccount, AccountStatus};
pub use user_session::{UserSession, SessionStatus};
pub use users::User;
