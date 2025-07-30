use worker::{Request, Response, RouteContext};
use crate::state::AppState;

/// General API route handlers
pub struct ApiRoutes;

impl ApiRoutes {
    /// Simple hello endpoint
    pub async fn hello(_req: Request, _ctx: RouteContext<AppState>) -> worker::Result<Response> {
        use serde_json::json;
        
        Response::from_json(&json!({
            "message": "Hello from API!",
            "service": "axum-worker",
            "version": "0.1.0"
        }))
    }
}