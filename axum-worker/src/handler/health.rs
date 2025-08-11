//! Health check handlers - Service monitoring and status endpoints

use axum::{extract::State, Json};
use serde::{ Serialize};
// Phase 1: Removing chrono import to fix getrandom conflicts
// use chrono::{DateTime, Utc};

use crate::{
    state::AppState,
    utils::error::AppError,
    database::test_connection::validate_connection_string,
};

/// Basic health check response - Phase 1: Simplified without time dependencies
#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String, // Phase 1: String instead of DateTime
}

/// Detailed health information - Phase 1: Simplified without time dependencies
#[derive(Serialize)]
pub struct ApiInfo {
    pub name: String,
    pub version: String,
    pub environment: String,
    pub timestamp: String, // Phase 1: String instead of DateTime
}

/// Database test response - Phase 1: Simplified without time dependencies
#[derive(Serialize)]
pub struct DatabaseTestResponse {
    pub database_connected: bool,
    pub timestamp: String, // Phase 1: String instead of DateTime
}

/// Basic health check endpoint
/// GET /api/health
pub async fn health_check() -> Result<Json<HealthResponse>, AppError> {
    let response = HealthResponse {
        status: "healthy".to_string(),
        timestamp: "2025-08-07T03:06:00Z".to_string(), // Phase 1: Static timestamp
    };
    
    Ok(Json(response))
}

/// API information endpoint
/// GET /api/info
pub async fn api_info(State(state): State<AppState>) -> Result<Json<ApiInfo>, AppError> {
    let response = ApiInfo {
        name: "Trading Dashboard API".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        environment: state.config.environment.clone(),
        timestamp: "2025-08-07T03:06:00Z".to_string(), // Phase 1: Static timestamp
    };
    
    Ok(Json(response))
}

/// Database connection test endpoint
/// GET /api/test/database
pub async fn database_test(State(state): State<AppState>) -> Result<Json<DatabaseTestResponse>, AppError> {
    // Use lazy initialization for database testing
    let database_connected = state.test_database_connection().await.is_ok();
    
    let response = DatabaseTestResponse {
        database_connected,
        timestamp: "2025-08-07T03:06:00Z".to_string(), // Phase 1: Static timestamp
    };
    
    Ok(Json(response))
}

/// Detailed database diagnostics endpoint
/// GET /api/test/database/detailed
pub async fn database_test_detailed(State(state): State<AppState>) -> Result<Json<serde_json::Value>, AppError> {
    worker::console_log!("Starting detailed database diagnostic...");

    let conn_str = state.config.database_url.clone();
    if conn_str.is_empty() {
        return Ok(Json(serde_json::json!({
            "success": false,
            "error": "No database URL configured",
            "timestamp": "2025-08-07T03:06:00Z"
        })));
    }

    // Validate connection string format and extract info
    let info = match validate_connection_string(&conn_str) {
        Ok(i) => {
            i.print_summary();
            i
        }
        Err(e) => {
            worker::console_log!("Database URL validation failed: {:?}", e);
            return Ok(Json(serde_json::json!({
                "success": false,
                "error": format!("{:?}", e),
                "timestamp": "2025-08-07T03:06:00Z"
            })));
        }
    };

    // Attempt an actual connection using the initialized client
    let connected = state.test_database_connection().await.is_ok();

    Ok(Json(serde_json::json!({
        "success": connected && info.is_valid,
        "database_connected": connected,
        "details": {
            "host": info.host,
            "port": info.port,
            "database": info.database,
            "username": info.username,
            "is_neon": info.is_neon,
            "has_ssl": info.has_ssl,
            "is_valid": info.is_valid,
        },
        "timestamp": "2025-08-07T03:06:00Z"
    })))
}