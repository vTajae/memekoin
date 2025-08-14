// Main router configuration - combines all route modules
use axum::Router;

use crate::{
    middleware::cors::CorsLayer,
    state::AppState
};

pub mod auth;
pub mod health;

/// Configure all routes by combining modular route configurations
pub async fn create_router() -> Result<Router<AppState>, crate::utils::error::AppError> {
    Ok(Router::new()
        // Combine all route modules
        .merge(health::routes())
        .merge(auth::routes())
        // Fallback for unmatched routes
        .fallback(fallback_handler)
        // Add CORS layer to allow frontend requests
        .layer(CorsLayer::new())
    )
}

/// Fallback handler for unmatched routes - serves frontend SPA
pub async fn fallback_handler(uri: axum::http::Uri) -> axum::response::Response<axum::body::Body> {
    use axum::response::{Html, IntoResponse};
    use axum::http::StatusCode;
    
    // If the path starts with /api, return a JSON 404 error
    if uri.path().starts_with("/api") {
        use serde_json::json;
        return (
            StatusCode::NOT_FOUND,
            axum::Json(json!({
                "success": false,
                "error": "API endpoint not found",
                "message": "This API endpoint does not exist"
            }))
        ).into_response();
    }
    
    // For all other routes, serve the frontend SPA (index.html)
    // This allows the frontend router to handle client-side routing
    let html = include_str!("../../static/index.html");
    Html(html).into_response()
}