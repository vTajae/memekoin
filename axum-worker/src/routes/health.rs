// Health check routes - URL mappings for health/monitoring endpoints
use axum::{
    Router,
    routing::get,
};

use crate::{
    handler::health,
    state::AppState,
};

/// Configure health check routes
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/health", get(health::health_check))
        .route("/api/info", get(health::api_info))
        .route("/api/test/database", get(health::database_test))
        .route("/api/test/database/detailed", get(health::database_test_detailed))
}