// Authentication routes - URL mappings for auth endpoints
use axum::{
    Router,
    routing::{get, post},
    middleware,
};

use crate::{
    handler::auth,
    handler::axiom,
    middleware::auth::{security_headers, auth_rate_limit},
    state::AppState,
};

/// Configure authentication routes
pub fn routes() -> Router<AppState> {
    Router::new()
        // LIVE DATA OAuth Flow - Redirect callback to frontend handler
        .route("/api/auth/oauth/login", get(auth::oauth_login_improved))
        .route("/api/auth/oauth/callback", get(auth::oauth_callback_redirect)) // Redirects to /auth/callback on the frontend
        .route("/api/auth/oauth/token", post(auth::handle_frontend_oauth)) // LIVE DATA: Frontend submits real Google tokens

    // Development-only token inspection (masked) to verify DB persistence
    .route("/api/auth/dev/google-token", get(auth::dev_google_token_info))
    .route("/api/auth/dev/gmail-axiom-code", get(auth::dev_gmail_axiom_code))
    .route("/api/auth/gmail/axiom-2fa", post(auth::gmail_axiom_2fa))



    // Axiom authentication handler (backend performs live Axiom calls)
    .route(
        "/api/auth/axiom",
        post(axiom::handle_axiom_auth),
    )


        // Protected routes with live session validation
        .route("/api/auth/user", get(auth::get_current_user_improved))
        .route("/api/auth/logout", post(auth::logout))

        // Apply security middleware to all auth routes
        .layer(middleware::from_fn(security_headers))
        .layer(middleware::from_fn(auth_rate_limit))
}