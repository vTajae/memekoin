//! Request handlers - HTTP request processing layer
//!
//! This module contains all HTTP request handlers for the live OAuth workflow.
//! Handlers are responsible for:
//! - Processing live Google OAuth data
//! - Managing user sessions with real data
//! - Formatting responses according to API standards

// Core OAuth handlers for live data flow
pub mod auth_improved;      // OAuth login and user management with live data
// pub mod oauth_callback;     // Disabled due to compilation issues
pub mod oauth_frontend;     // Frontend OAuth token handling with live data

// Utility handlers
pub mod health;             // Health check endpoint