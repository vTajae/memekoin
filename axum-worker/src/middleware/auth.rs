//! Basic auth-related middleware stubs
//! You can expand these with real checks and rate-limiting logic.

use axum::{middleware::Next, response::IntoResponse};
use axum::http::HeaderValue;
use axum::extract::Request; // This is a type alias for http::Request<Body> in axum 0.8

/// Adds security headers. Extend as needed.
pub async fn security_headers(req: Request, next: Next) -> impl IntoResponse {
    let mut res = next.run(req).await;
    let headers = res.headers_mut();
    headers.insert("X-Content-Type-Options", HeaderValue::from_static("nosniff"));
    headers.insert("X-Frame-Options", HeaderValue::from_static("DENY"));
    headers.insert("Referrer-Policy", HeaderValue::from_static("no-referrer"));
    headers.insert("Cross-Origin-Opener-Policy", HeaderValue::from_static("same-origin"));
    res
}

/// Simple pass-through rate limit placeholder. Replace with real logic.
pub async fn auth_rate_limit(req: Request, next: Next) -> impl IntoResponse {
    next.run(req).await
}

pub async fn require_auth(req: Request, next: Next) -> impl IntoResponse {
    // TODO: implement checks using tower-sessions Session extractor if needed
    next.run(req).await
}

pub async fn optional_auth(req: Request, next: Next) -> impl IntoResponse {
    next.run(req).await
}
