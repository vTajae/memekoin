use worker::{Request, Response, RouteContext};
use serde_json::json;
use crate::models::{LoginRequest, RegisterRequest};
use crate::state::AppState;

/// Authentication route handlers
pub struct AuthRoutes;

impl AuthRoutes {
    /// Handle user login
    pub async fn login(
        mut req: Request,
        ctx: RouteContext<AppState>
    ) -> worker::Result<Response> {
        let payload: LoginRequest = match req.json().await {
            Ok(p) => p,
            Err(e) => return Response::error(format!("Invalid JSON: {}", e), 400),
        };

        // Validate input
        if payload.username.trim().is_empty() || payload.password.trim().is_empty() {
            return Response::error("Username and password are required", 400);
        }

        // Process login through service
        match ctx.data.auth_service.user_service().login(payload).await {
            Ok(auth_response) => {
                let status = if auth_response.success { 200 } else { 401 };
                Ok(Response::from_json(&auth_response)?.with_status(status))
            }
            Err(e) => Response::error(format!("Login failed: {}", e), 500),
        }
    }

    /// Handle user registration
    pub async fn register(
        mut req: Request,
        ctx: RouteContext<AppState>
    ) -> worker::Result<Response> {
        let payload: RegisterRequest = match req.json().await {
            Ok(p) => p,
            Err(e) => return Response::error(format!("Invalid JSON: {}", e), 400),
        };

        // Validate input
        if payload.username.trim().is_empty() || 
           payload.email.trim().is_empty() || 
           payload.password.trim().is_empty() {
            return Response::error("Username, email, and password are required", 400);
        }

        if payload.password.len() < 6 {
            return Response::error("Password must be at least 6 characters", 400);
        }

        // Process registration through service
        match ctx.data.auth_service.user_service().register(payload).await {
            Ok(auth_response) => {
                let status = if auth_response.success { 201 } else { 400 };
                Ok(Response::from_json(&auth_response)?.with_status(status))
            }
            Err(e) => Response::error(format!("Registration failed: {}", e), 500),
        }
    }

    /// Handle getting current user info
    pub async fn me(
        req: Request,
        ctx: RouteContext<AppState>
    ) -> worker::Result<Response> {
        // Extract token from Authorization header
        let token = match Self::extract_bearer_token(&req) {
            Some(t) => t,
            None => return Response::error("Authorization header required", 401),
        };

        // Get current user through service
        match ctx.data.auth_service.authenticate_token(&token).await {
            Ok(user_info) => Response::from_json(&json!({
                "success": true,
                "user": user_info
            })),
            Err(e) => Response::error(format!("Authentication failed: {}", e), 401),
        }
    }

    /// Extract Bearer token from Authorization header
    fn extract_bearer_token(req: &Request) -> Option<String> {
        let auth_header = req.headers().get("Authorization").ok()??;
        
        if auth_header.starts_with("Bearer ") {
            Some(auth_header[7..].to_string())
        } else {
            None
        }
    }
}

