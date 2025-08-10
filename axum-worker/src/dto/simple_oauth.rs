//! Data Transfer Objects - Request/Response types

use serde::{Deserialize, Serialize};
// Phase 1: Removing uuid import to fix getrandom conflicts
// use uuid::Uuid;

/// Authentication-related DTOs
pub mod auth {
    use super::*;

    /// Google OAuth callback request
    #[derive(Debug, Deserialize)]
    pub struct GoogleOAuthCallback {
        pub code: String,
        pub state: Option<String>,
        pub error: Option<String>,
        pub error_description: Option<String>,
    }

    /// OAuth token exchange request
    #[derive(Debug, Deserialize)]
    pub struct OAuthTokenExchange {
        pub code: String,
        pub state: Option<String>,
    }

    /// Login request
    #[derive(Debug, Deserialize)]
    pub struct LoginRequest {
        pub email: String,
        pub password: String,
    }

    /// Registration request
    #[derive(Debug, Deserialize)]
    pub struct RegisterRequest {
        pub email: String,
        pub password: String,
        pub first_name: Option<String>,
        pub last_name: Option<String>,
    }

    /// Complete onboarding request
    #[derive(Debug, Deserialize)]
    pub struct CompleteOnboardingRequest {
        pub first_name: String,
        pub last_name: String,
    }

    /// Authentication response - Phase 1: Simplified without time dependencies
    #[derive(Debug, Serialize)]
    pub struct AuthResponse {
        pub user: UserResponse,
        pub token: Option<String>,
        pub expires_at: Option<String>, // Phase 1: String instead of DateTime
    }

    /// User response - Phase 1: Simplified without uuid/chrono dependencies
    #[derive(Debug, Serialize, Clone)]
    pub struct UserResponse {
        pub id: String, // Phase 1: String instead of Uuid
        pub email: String,
        pub first_name: Option<String>,
        pub last_name: Option<String>,
        pub is_oauth: bool,
        pub created_at: String, // Phase 1: String instead of DateTime
    }

    /// OAuth status response
    #[derive(Debug, Serialize)]
    pub struct OAuthStatusResponse {
        pub is_authenticated: bool,
        pub user: Option<UserResponse>,
        pub oauth_url: Option<String>,
    }

    /// Google OAuth user info from Google API
    #[derive(Debug, Deserialize)]
    pub struct GoogleUserInfo {
        pub id: String,
        pub email: String,
        pub verified_email: bool,
        pub name: Option<String>,
        pub given_name: Option<String>,
        pub family_name: Option<String>,
        pub picture: Option<String>,
        pub locale: Option<String>,
    }

    /// Google OAuth token response
    #[derive(Debug, Deserialize)]
    pub struct GoogleTokenResponse {
        pub access_token: String,
        pub token_type: String,
        pub expires_in: i64,
        pub refresh_token: Option<String>,
        pub scope: String,
        pub id_token: Option<String>,
    }
}

/// OAuth-specific DTOs for Phase 2 
pub mod oauth;

/// Standard API response types - Phase 3 enhancement
pub mod response;

/// Health check response - Phase 1: Simplified without chrono dependency
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String, // Phase 1: String instead of DateTime
}
