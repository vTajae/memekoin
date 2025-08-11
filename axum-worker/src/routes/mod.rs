// Main router configuration - combines all route modules
use axum::Router;

use crate::{
    // middleware::cors::CorsLayer, // Phase 1: Commenting out complex middleware
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
    )
}

/// Fallback handler for unmatched routes
pub async fn fallback_handler() -> axum::Json<serde_json::Value> {
    use serde_json::json;
    
    axum::Json(json!({
        "success": false,
        "error": "Endpoint not found",
        "message": "This endpoint does not exist"
    }))
}