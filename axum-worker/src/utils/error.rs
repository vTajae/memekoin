//! Error handling module - Centralized error types and handling

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use std::fmt;

use crate::dto::response::ApiError;

/// Application-wide error type
#[derive(Debug)]
pub enum AppError {
    /// Authentication/Authorization errors
    Auth(AuthError),
    /// Database operation errors
    Database(String),
    /// Database connection errors
    DatabaseError(String),
    /// External service errors (OAuth, APIs)
    ExternalService(String),
    /// Validation errors
    Validation(String),
    /// Internal server errors
    Internal(String),
    /// Configuration errors
    Config(String),
    /// Serialization/Deserialization errors
    SerializationError(String),
    /// Authentication errors (general)
    AuthenticationError(String),
    /// External service error (shorter alias for ExternalService)
    ExternalServiceError(String),
    /// Validation error (shorter alias for Validation)
    ValidationError(String),
    /// Internal server error (shorter alias for Internal)
    InternalServerError(String),
    /// Not found errors
    NotFound(String),
    /// Bad request errors
    BadRequest(String),
}

/// Authentication-specific errors
#[derive(Debug)]
pub enum AuthError {
    /// Invalid credentials
    InvalidCredentials,
    /// Missing authentication token
    MissingToken,
    /// Invalid or expired token
    InvalidToken,
    /// OAuth flow errors
    OAuthError(String),
    /// Session errors
    SessionError(String),
    /// User not found
    UserNotFound,
    /// Insufficient permissions
    Forbidden,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Auth(e) => write!(f, "Authentication error: {}", e),
            AppError::Database(msg) => write!(f, "Database error: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "Database connection error: {}", msg),
            AppError::ExternalService(msg) => write!(f, "External service error: {}", msg),
            AppError::Validation(msg) => write!(f, "Validation error: {}", msg),
            AppError::Internal(msg) => write!(f, "Internal error: {}", msg),
            AppError::Config(msg) => write!(f, "Configuration error: {}", msg),
            AppError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            AppError::AuthenticationError(msg) => write!(f, "Authentication error: {}", msg),
            AppError::ExternalServiceError(msg) => write!(f, "External service error: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::InternalServerError(msg) => write!(f, "Internal server error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
        }
    }
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::InvalidCredentials => write!(f, "Invalid credentials"),
            AuthError::MissingToken => write!(f, "Missing authentication token"),
            AuthError::InvalidToken => write!(f, "Invalid or expired token"),
            AuthError::OAuthError(msg) => write!(f, "OAuth error: {}", msg),
            AuthError::SessionError(msg) => write!(f, "Session error: {}", msg),
            AuthError::UserNotFound => write!(f, "User not found"),
            AuthError::Forbidden => write!(f, "Insufficient permissions"),
        }
    }
}

impl std::error::Error for AppError {}
impl std::error::Error for AuthError {}

impl AppError {
    /// Get the standardized error code for this error
    pub fn error_code(&self) -> &'static str {
        match self {
            AppError::Auth(AuthError::InvalidCredentials) => "AUTH_INVALID_CREDENTIALS",
            AppError::Auth(AuthError::MissingToken) => "AUTH_MISSING_TOKEN", 
            AppError::Auth(AuthError::InvalidToken) => "AUTH_INVALID_TOKEN",
            AppError::Auth(AuthError::UserNotFound) => "AUTH_USER_NOT_FOUND",
            AppError::Auth(AuthError::Forbidden) => "AUTH_FORBIDDEN",
            AppError::Auth(AuthError::OAuthError(_)) => "AUTH_OAUTH_ERROR",
            AppError::Auth(AuthError::SessionError(_)) => "AUTH_SESSION_ERROR",
            AppError::Validation(_) | AppError::ValidationError(_) => "VALIDATION_ERROR",
            AppError::Database(_) | AppError::DatabaseError(_) => "DATABASE_ERROR",
            AppError::ExternalService(_) | AppError::ExternalServiceError(_) => "EXTERNAL_SERVICE_ERROR",
            AppError::Config(_) => "CONFIG_ERROR",
            AppError::Internal(_) | AppError::InternalServerError(_) => "INTERNAL_SERVER_ERROR",
            AppError::SerializationError(_) => "SERIALIZATION_ERROR",
            AppError::AuthenticationError(_) => "AUTH_ERROR",
            AppError::NotFound(_) => "RESOURCE_NOT_FOUND",
            AppError::BadRequest(_) => "BAD_REQUEST",
        }
    }

    /// Get the status code and human-readable message for this error
    pub fn status_and_message(&self) -> (StatusCode, &'static str) {
        match self {
            AppError::Auth(AuthError::InvalidCredentials) => {
                (StatusCode::UNAUTHORIZED, "Invalid credentials provided")
            }
            AppError::Auth(AuthError::MissingToken) => {
                (StatusCode::UNAUTHORIZED, "Authentication token is missing")
            }
            AppError::Auth(AuthError::InvalidToken) => {
                (StatusCode::UNAUTHORIZED, "Authentication token is invalid or expired")
            }
            AppError::Auth(AuthError::UserNotFound) => {
                (StatusCode::NOT_FOUND, "User not found")
            }
            AppError::Auth(AuthError::Forbidden) => {
                (StatusCode::FORBIDDEN, "Insufficient permissions")
            }
            AppError::Auth(AuthError::OAuthError(_)) => {
                (StatusCode::BAD_REQUEST, "OAuth authentication failed")
            }
            AppError::Auth(AuthError::SessionError(_)) => {
                (StatusCode::UNAUTHORIZED, "Session error occurred")
            }
            AppError::Validation(_) | AppError::ValidationError(_) => {
                (StatusCode::BAD_REQUEST, "Validation failed")
            }
            AppError::Database(_) | AppError::DatabaseError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Database operation failed")
            }
            AppError::ExternalService(_) | AppError::ExternalServiceError(_) => {
                (StatusCode::BAD_GATEWAY, "External service unavailable")
            }
            AppError::Config(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Configuration error")
            }
            AppError::Internal(_) | AppError::InternalServerError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            AppError::SerializationError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Data serialization error")
            }
            AppError::AuthenticationError(_) => {
                (StatusCode::UNAUTHORIZED, "Authentication failed")
            }
            AppError::NotFound(_) => {
                (StatusCode::NOT_FOUND, "Resource not found")
            }
            AppError::BadRequest(_) => {
                (StatusCode::BAD_REQUEST, "Bad request")
            }
        }
    }
}

/// Convert AppError into HTTP Response with standardized ApiError format
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = self.status_and_message();
        
        let body = Json(ApiError::new(
            self.error_code(),
            message,
            Some(self.to_string()),
        ));

        (status, body).into_response()
    }
}

/// Helper trait for converting Results into AppError
pub trait ResultExt<T> {
    fn map_internal_err(self, msg: &str) -> Result<T, AppError>;
    fn map_config_err(self, msg: &str) -> Result<T, AppError>;
    fn map_database_err(self, msg: &str) -> Result<T, AppError>;
}

impl<T, E: std::error::Error> ResultExt<T> for Result<T, E> {
    fn map_internal_err(self, msg: &str) -> Result<T, AppError> {
        self.map_err(|e| AppError::Internal(format!("{}: {}", msg, e)))
    }

    fn map_config_err(self, msg: &str) -> Result<T, AppError> {
        self.map_err(|e| AppError::Config(format!("{}: {}", msg, e)))
    }

    fn map_database_err(self, msg: &str) -> Result<T, AppError> {
        self.map_err(|e| AppError::Database(format!("{}: {}", msg, e)))
    }
}

/// Convert worker::Error to AppError
impl From<worker::Error> for AppError {
    fn from(error: worker::Error) -> Self {
        AppError::ExternalService(format!("Worker error: {}", error))
    }
}
