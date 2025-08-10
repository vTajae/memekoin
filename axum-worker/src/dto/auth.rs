//! Phase 3 Authentication DTOs - Enhanced request/response types for auth endpoints

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// OAuth login request parameters
#[derive(Debug, Deserialize)]
pub struct OAuthLoginParams {
    /// Optional URL to redirect to after successful authentication
    pub redirect_after: Option<String>,
}

/// OAuth callback parameters from Google
#[derive(Debug, Deserialize)]
pub struct OAuthCallbackParams {
    /// Authorization code from Google
    pub code: String,
    /// CSRF protection state parameter
    pub state: String,
    /// Error code if authorization failed
    pub error: Option<String>,
    /// Human-readable error description
    pub error_description: Option<String>,
}

impl OAuthCallbackParams {
    /// Check if this callback represents an error response
    pub fn is_error(&self) -> bool {
        self.error.is_some()
    }

    /// Get formatted error message from callback parameters
    pub fn error_message(&self) -> Option<String> {
        match (&self.error, &self.error_description) {
            (Some(error), Some(description)) => Some(format!("{}: {}", error, description)),
            (Some(error), None) => Some(error.clone()),
            (None, Some(description)) => Some(description.clone()),
            (None, None) => None,
        }
    }
}

/// Token refresh request (future implementation)
#[derive(Debug, Deserialize)]
pub struct TokenRefreshRequest {
    /// Refresh token to use for generating new access token
    pub refresh_token: String,
}

/// Token refresh response
#[derive(Debug, Serialize)]
pub struct TokenRefreshResponse {
    /// Token lifetime in seconds
    pub expires_in: u32,
    /// Token type (typically "Bearer")
    pub token_type: String,
}

/// User profile response for authenticated users
#[derive(Debug, Serialize, Clone)]
pub struct UserProfileResponse {
    /// User information
    pub user: UserResponse,
    /// Session expiration time
    pub session_expires_at: DateTime<Utc>,
    /// Current session ID
    pub session_id: String,
}

/// Enhanced user response with complete profile information
#[derive(Debug, Serialize, Clone)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub verified_email: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
}

/// User session information response
#[derive(Debug, Serialize)]
pub struct UserSessionResponse {
    pub session_id: String,
    pub created_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
    pub is_current: bool,
}

/// List of user sessions response
#[derive(Debug, Serialize)]
pub struct UserSessionsResponse {
    pub sessions: Vec<UserSessionResponse>,
    pub total_count: usize,
}

/// Logout response
#[derive(Debug, Serialize)]
pub struct LogoutResponse {
    pub message: String,
    pub logged_out_at: DateTime<Utc>,
}

/// Authentication status response
#[derive(Debug, Serialize)]
pub struct AuthStatusResponse {
    pub authenticated: bool,
    pub user: Option<UserResponse>,
    pub session_expires_at: Option<DateTime<Utc>>,
}

/// Simple authentication response for basic endpoints
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub message: String,
}

impl From<String> for AuthResponse {
    fn from(message: String) -> Self {
        Self { message }
    }
}

impl From<&str> for AuthResponse {
    fn from(message: &str) -> Self {
        Self { message: message.to_string() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oauth_callback_error_detection() {
        let error_callback = OAuthCallbackParams {
            code: "".to_string(),
            state: "test-state".to_string(),
            error: Some("access_denied".to_string()),
            error_description: Some("User denied access".to_string()),
        };

        assert!(error_callback.is_error());
        assert_eq!(error_callback.error_message(), Some("access_denied: User denied access".to_string()));

        let success_callback = OAuthCallbackParams {
            code: "auth_code_123".to_string(),
            state: "test-state".to_string(),
            error: None,
            error_description: None,
        };

        assert!(!success_callback.is_error());
        assert_eq!(success_callback.error_message(), None);
    }

    #[test]
    fn test_auth_response_creation() {
        let response1 = AuthResponse::from("Test message");
        assert_eq!(response1.message, "Test message");

        let response2 = AuthResponse::from("Another message".to_string());
        assert_eq!(response2.message, "Another message");
    }
}