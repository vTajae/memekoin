//! Standard API response types for consistent JSON responses across all endpoints

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Standard success response format for all API endpoints
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub timestamp: DateTime<Utc>,
}

/// Standard error response format for all API endpoints
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiError {
    pub success: bool,
    pub error: ErrorDetails,
    pub timestamp: DateTime<Utc>,
}

/// Error details structure for consistent error reporting
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorDetails {
    pub code: String,           // ERROR_CODE_FORMAT
    pub message: String,        // Human-readable message
    pub details: Option<String>, // Technical details for debugging
}

impl<T> ApiResponse<T> {
    /// Create a successful response with data
    pub fn success(data: Option<T>, message: Option<&str>) -> Self {
        Self {
            success: true,
            data,
            message: message.map(|s| s.to_string()),
            timestamp: Utc::now(),
        }
    }

    /// Create a successful response with just a message
    pub fn success_with_message(message: &str) -> ApiResponse<()> {
        ApiResponse {
            success: true,
            data: None,
            message: Some(message.to_string()),
            timestamp: Utc::now(),
        }
    }

    /// Create a successful response with data and message
    pub fn success_with_data(data: T, message: Option<&str>) -> Self {
        Self::success(Some(data), message)
    }
}

impl ApiError {
    /// Create a new error response
    pub fn new(code: &str, message: &str, details: Option<String>) -> Self {
        Self {
            success: false,
            error: ErrorDetails {
                code: code.to_string(),
                message: message.to_string(),
                details,
            },
            timestamp: Utc::now(),
        }
    }

    /// Create authentication error
    pub fn auth_error(message: &str, details: Option<String>) -> Self {
        Self::new("AUTH_ERROR", message, details)
    }

    /// Create validation error
    pub fn validation_error(message: &str, details: Option<String>) -> Self {
        Self::new("VALIDATION_ERROR", message, details)
    }

    /// Create database error
    pub fn database_error(message: &str, details: Option<String>) -> Self {
        Self::new("DATABASE_ERROR", message, details)
    }

    /// Create internal server error
    pub fn internal_error(message: &str, details: Option<String>) -> Self {
        Self::new("INTERNAL_SERVER_ERROR", message, details)
    }
}