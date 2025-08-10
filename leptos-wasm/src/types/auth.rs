#![allow(dead_code)]
use serde::{Deserialize, Serialize};

/// User information from authentication system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub picture: Option<String>,
}

/// Authentication state for reactive components
#[derive(Debug, Clone, PartialEq)]
pub enum AuthState {
    Loading,
    Authenticated(User),
    Unauthenticated,
}

/// Standard API response wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

/// API error response structure
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub success: bool,
    pub error: ErrorDetails,
}

/// Detailed error information
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetails {
    pub code: String,
    pub message: String,
    pub details: Option<String>,
}

impl AuthState {
    /// Check if user is authenticated
    pub fn is_authenticated(&self) -> bool {
        matches!(self, AuthState::Authenticated(_))
    }

    /// Get user data if authenticated
    pub fn user(&self) -> Option<&User> {
        match self {
            AuthState::Authenticated(user) => Some(user),
            _ => None,
        }
    }

    /// Check if currently loading
    pub fn is_loading(&self) -> bool {
        matches!(self, AuthState::Loading)
    }
}

/// Google OAuth token response
#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleTokenResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
    pub token_type: String,
}

/// Google user info from OAuth API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleUserInfo {
    #[serde(alias = "sub")]
    pub id: String,  // Google returns 'id' in v2 API, 'sub' in OpenID Connect
    pub email: String,
    #[serde(default)]
    pub email_verified: bool,
    pub name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub picture: Option<String>,
    pub locale: Option<String>,
}

/// OAuth token submission to backend
#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthTokenSubmission {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: i64,
    pub state: String,
    pub user_info: GoogleUserInfo,
    pub code: Option<String>, // Authorization code from OAuth callback
}

/// OAuth token response from backend
#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthTokenResponse {
    pub success: bool,
    pub session_id: String,
    pub user_email: String,
    pub expires_at: String,
}