//! Simplified Authentication Service for Phase 1
//! This is a minimal implementation to get compilation working
//! Full OAuth implementation will be added in Phase 2

use crate::{
    dto::oauth::{GoogleTokenResponse, GoogleUserInfo},
    dto::auth::UserResponse,
    repository::user::User,
    error::{AppError, AuthError},
    state::GoogleOAuthConfig,
};
use uuid::Uuid;
use chrono::Utc;

/// Simplified authentication service for Phase 1
pub struct AuthService {
    oauth_config: GoogleOAuthConfig,
}

impl AuthService {
    /// Create new authentication service
    pub fn new(oauth_config: GoogleOAuthConfig) -> Self {
        Self { oauth_config }
    }

    /// Build OAuth authorization URL (placeholder for Phase 1)
    pub fn build_oauth_url(&self, state: &str, code_challenge: &str) -> Result<String, AppError> {
        Ok(format!(
            "{}?client_id={}&response_type=code&state={}&code_challenge={}",
            self.oauth_config.auth_url,
            self.oauth_config.client_id,
            state,
            code_challenge
        ))
    }

    /// Exchange OAuth code for access token (placeholder for Phase 1)
    pub async fn exchange_oauth_code(
        &self,
        _code: &str,
        _code_verifier: &str,
    ) -> Result<GoogleTokenResponse, AppError> {
        Ok(GoogleTokenResponse {
            access_token: "mock_token".to_string(),
            token_type: "Bearer".to_string(),
            expires_in: 3600,
            refresh_token: None,
            scope: Some("openid email profile".to_string()),
        })
    }

    /// Get user info (placeholder for Phase 1)
    pub async fn get_google_user_info(&self, _access_token: &str) -> Result<GoogleUserInfo, AppError> {
        Ok(GoogleUserInfo {
            sub: "mock_user_id".to_string(),
            email: "user@example.com".to_string(),
            email_verified: true,
            name: Some("Test User".to_string()),
            given_name: Some("Test".to_string()),
            family_name: Some("User".to_string()),
            picture: None,
            locale: Some("en".to_string()),
        })
    }

    /// Create or update user (simplified for Phase 1)
    pub async fn create_or_update_oauth_user(&self, google_user: GoogleUserInfo) -> Result<User, AppError> {
        let user = User {
            id: Uuid::new_v4(), // Generate new UUID
            email: google_user.email,
            first_name: google_user.given_name,
            last_name: google_user.family_name,
            password: None, // OAuth user, no password
            created_at: Utc::now(),
        };
        Ok(user)
    }

    /// Validate credentials (placeholder for Phase 1)
    pub async fn validate_credentials(&self, _email: &str, _password: &str) -> Result<User, AppError> {
        Err(AppError::Auth(AuthError::InvalidCredentials))
    }

    /// Register user (simplified for Phase 1)
    pub async fn register_user(
        &self,
        email: String,
        _password: String,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> Result<User, AppError> {
        let user = User {
            id: Uuid::new_v4(), // Generate new UUID
            email,
            first_name,
            last_name,
            password: None, // For now, no password handling in Phase 1
            created_at: Utc::now(),
        };
        Ok(user)
    }
}

/// Convert User to UserResponse DTO (simplified for Phase 1)
impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        // Combine first and last name for the display name
        let name = match (&user.first_name, &user.last_name) {
            (Some(first), Some(last)) => Some(format!("{} {}", first, last)),
            (Some(first), None) => Some(first.clone()),
            (None, Some(last)) => Some(last.clone()),
            (None, None) => None,
        };
        
        Self {
            id: user.id.to_string(), // Convert UUID to String
            email: user.email.clone(),
            name,
            picture: None, // Will be populated from OAuth profile
            verified_email: user.password.is_none(), // OAuth users are verified
            created_at: user.created_at,
            updated_at: user.created_at, // Using created_at as placeholder
            last_login: None, // Not tracked in simplified version
        }
    }
}