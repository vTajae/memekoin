//! Middleware module - Cross-cutting concerns

// Phase 2: Authentication middleware for secure OAuth flows
pub mod auth;
pub mod cors;
pub mod session; // simple cookie-based session management

// Re-export middleware for convenience
pub use auth::{require_auth, optional_auth, security_headers, auth_rate_limit};
pub use cors::CorsLayer;
pub use session::{SimpleSessionResponse, simple_session_setup};