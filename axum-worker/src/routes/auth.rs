// Authentication routes - URL mappings for auth endpoints
use axum::{
    Router,
    routing::{get, post},
    middleware,
};

use crate::{
    handler::{auth_improved, oauth_frontend},
    middleware::auth::{security_headers, auth_rate_limit},
    state::AppState,
};

/// Configure authentication routes
pub fn routes() -> Router<AppState> {
    Router::new()
        // LIVE DATA OAuth Flow - Redirect callback to frontend handler
        .route("/api/auth/oauth/login", get(auth_improved::oauth_login_improved))
        .route("/api/auth/oauth/callback", get(auth_improved::oauth_callback_redirect)) // Redirects to /auth/callback on the frontend
        .route("/api/auth/oauth/token", post(oauth_frontend::handle_frontend_oauth)) // LIVE DATA: Frontend submits real Google tokens

        // Protected routes with live session validation
        .route("/api/auth/user", get(auth_improved::get_current_user_improved))

        // Apply security middleware to all auth routes
        .layer(middleware::from_fn(security_headers))
        .layer(middleware::from_fn(auth_rate_limit))
}