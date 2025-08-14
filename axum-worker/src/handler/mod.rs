//! Request handlers - HTTP request processing layer
//!
//! This module contains all HTTP request handlers for the live OAuth workflow.
//! Handlers are responsible for:
//! - Processing live Google OAuth data
//! - Managing user sessions with real data
//! - Formatting responses according to API standards

// Core OAuth handlers for live data flow
pub mod auth;      // OAuth login and user management with live data
// Utility handlers
pub mod health;             // Health check endpoint
pub mod axiom;         // Axiom Trade authentication with Gmail 2FA