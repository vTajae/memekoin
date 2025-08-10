//! Services module - Business logic layer

// Phase 1: Use simplified auth service
pub mod auth_simple;

// Phase 2: OAuth services aligned with database schema
// pub mod oauth;
pub mod oauth_simple;
pub mod session;

// Phase 2: Will re-enable full auth service
// pub mod auth;
