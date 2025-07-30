use worker::{Router, Request, Response, RouteContext, Url};
use crate::state::AppState;

/// Create the main application router with all routes configured
pub fn create_router(app_state: AppState) -> Router<'static, AppState> {
    Router::with_data(app_state)
        // Health check and basic routes
        .get("/", root_handler)
        .get("/api/health", sync_health_handler)

        // Authentication routes (sync versions to avoid WASM closure issues)
        .post("/api/auth/login", sync_login_handler)
        .post("/api/auth/register", sync_register_handler)
}

/// Root handler - redirect to client app
fn root_handler(req: Request, _ctx: RouteContext<AppState>) -> worker::Result<Response> {
    let url = req.url()?;
    let redirect_url = Url::parse(&format!("{}://{}/app", url.scheme(), url.host_str().unwrap_or("localhost")))?;
    Response::redirect(redirect_url)
}

/// Sync health check endpoint
fn sync_health_handler(_req: Request, _ctx: RouteContext<AppState>) -> worker::Result<Response> {
    use serde_json::json;

    Response::from_json(&json!({
        "status": "healthy",
        "service": "axum-worker",
        "timestamp": "2025-07-29T22:00:00Z",
        "architecture": "repo-server-route"
    }))
}

/// Sync login handler
fn sync_login_handler(_req: Request, _ctx: RouteContext<AppState>) -> worker::Result<Response> {
    use serde_json::json;

    // For now, return a simple success response for demo user
    Response::from_json(&json!({
        "success": true,
        "message": "Login successful",
        "token": "demo-jwt-token",
        "user": {
            "id": "demo-id",
            "username": "demo",
            "email": "demo@example.com"
        }
    }))
}

/// Sync register handler
fn sync_register_handler(_req: Request, _ctx: RouteContext<AppState>) -> worker::Result<Response> {
    use serde_json::json;

    // For now, return a simple success response
    Response::from_json(&json!({
        "success": true,
        "message": "Registration successful",
        "token": "demo-jwt-token",
        "user": {
            "id": "new-user-id",
            "username": "newuser",
            "email": "newuser@example.com"
        }
    }))
}

/// Health check endpoint
async fn health_handler(_req: Request, _ctx: RouteContext<AppState>) -> worker::Result<Response> {
    use serde_json::json;

    Response::from_json(&json!({
        "status": "healthy",
        "service": "axum-worker",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "architecture": "repo-server-route"
    }))
}