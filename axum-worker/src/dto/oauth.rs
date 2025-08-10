//! OAuth DTOs - Data Transfer Objects for Phase 2 OAuth 2.0 implementation

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// OAuth state parameter for CSRF protection and PKCE flow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthState {
    pub state: String,                 // CSRF protection token
    pub code_verifier: String,         // PKCE code verifier
    pub code_challenge: String,        // PKCE code challenge
    pub redirect_after_login: Option<String>, // Deep linking support
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

impl OAuthState {
    pub fn new(
        state: String,
        code_verifier: String,
        code_challenge: String,
        redirect_after_login: Option<String>,
        ttl_seconds: i64,
    ) -> Self {
        let now = chrono::Utc::now();
        Self {
            state,
            code_verifier,
            code_challenge,
            redirect_after_login,
            created_at: now,
            expires_at: now + chrono::Duration::seconds(ttl_seconds),
        }
    }

    pub fn is_expired(&self) -> bool {
        chrono::Utc::now() > self.expires_at
    }

    pub fn is_valid(&self) -> bool {
        !self.is_expired()
    }
}

/// Google OAuth token response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleTokenResponse {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: i64,
    pub token_type: String,
    pub scope: Option<String>,
}

impl GoogleTokenResponse {
    pub fn expires_at(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::Utc::now() + chrono::Duration::seconds(self.expires_in)
    }

    pub fn is_bearer_token(&self) -> bool {
        self.token_type.to_lowercase() == "bearer"
    }
}

/// Google user info from OAuth userinfo endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoogleUserInfo {
    pub sub: String,                   // Google user ID
    pub email: String,
    pub email_verified: bool,
    pub name: Option<String>,
    pub given_name: Option<String>,    // First name
    pub family_name: Option<String>,   // Last name
    pub picture: Option<String>,       // Avatar URL
    pub locale: Option<String>,
}

impl GoogleUserInfo {
    pub fn first_name(&self) -> Option<&str> {
        self.given_name.as_deref()
    }

    pub fn last_name(&self) -> Option<&str> {
        self.family_name.as_deref()
    }

    pub fn provider_user_id(&self) -> &str {
        &self.sub
    }

    pub fn is_email_verified(&self) -> bool {
        self.email_verified
    }
}

/// OAuth login request parameters
#[derive(Debug, Deserialize)]
pub struct OAuthLoginParams {
    pub redirect_after: Option<String>, // Where to redirect after successful login
}

/// OAuth callback parameters from Google
#[derive(Debug, Deserialize)]
pub struct OAuthCallbackParams {
    pub code: String,                  // Authorization code
    pub state: String,                 // CSRF state parameter
    pub scope: Option<String>,         // Granted scopes
    pub error: Option<String>,         // Error code if authorization failed
    pub error_description: Option<String>, // Error description
}

impl OAuthCallbackParams {
    pub fn is_success(&self) -> bool {
        self.error.is_none()
    }

    pub fn is_error(&self) -> bool {
        self.error.is_some()
    }

    pub fn error_message(&self) -> Option<String> {
        match (&self.error, &self.error_description) {
            (Some(error), Some(desc)) => Some(format!("{}: {}", error, desc)),
            (Some(error), None) => Some(error.clone()),
            _ => None,
        }
    }
}

/// Token refresh request
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenRefreshRequest {
    pub refresh_token: String,
}

/// Token refresh response
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenRefreshResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub token_type: String,
    pub scope: Option<String>,
    pub refresh_token: Option<String>, // New refresh token if rotated
}

impl TokenRefreshResponse {
    pub fn expires_at(&self) -> chrono::DateTime<chrono::Utc> {
        chrono::Utc::now() + chrono::Duration::seconds(self.expires_in)
    }
}

/// User session creation request
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserSessionRequest {
    pub user_id: Uuid,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub token_expires_at: chrono::DateTime<chrono::Utc>,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
}

/// User session response
#[derive(Debug, Serialize, Deserialize)]
pub struct UserSessionResponse {
    pub session_id: String,
    pub user_id: Uuid,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl UserSessionResponse {
    pub fn from_session(session: &crate::entity::UserSession) -> Self {
        Self {
            session_id: session.session_id().to_string(),
            user_id: session.user_id(),
            expires_at: session.expires_at(),
            created_at: session.created_at,
        }
    }
}

/// OAuth provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthProviderConfig {
    pub provider_id: i16,
    pub provider_name: String,
    pub client_id: String,
    pub client_secret: String,       // Should be encrypted in storage
    pub redirect_uri: String,
    pub auth_url: String,
    pub token_url: String,
    pub userinfo_url: String,
    pub scopes: Vec<String>,
    pub code_challenge_method: String, // "S256" for PKCE
}

impl OAuthProviderConfig {
    pub fn google_config(
        client_id: String,
        client_secret: String,
        redirect_uri: String,
    ) -> Self {
        Self {
            provider_id: 2, // Assuming provider ID 2 is Google
            provider_name: "google".to_string(),
            client_id,
            client_secret,
            redirect_uri,
            auth_url: "https://accounts.google.com/o/oauth2/auth".to_string(),
            token_url: "https://oauth2.googleapis.com/token".to_string(),
            userinfo_url: "https://www.googleapis.com/oauth2/v2/userinfo".to_string(),
            scopes: vec![
                "openid".to_string(),
                "email".to_string(),
                "profile".to_string(),
                "https://www.googleapis.com/auth/gmail.readonly".to_string(),
            ],
            code_challenge_method: "S256".to_string(),
        }
    }

    pub fn get_scope_string(&self) -> String {
        self.scopes.join(" ")
    }

    pub fn is_google(&self) -> bool {
        self.provider_name == "google"
    }
}

/// OAuth error response
#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthErrorResponse {
    pub error: String,
    pub error_description: Option<String>,
    pub error_uri: Option<String>,
    pub state: Option<String>,
}

impl OAuthErrorResponse {
    pub fn new(error: String, description: Option<String>) -> Self {
        Self {
            error,
            error_description: description,
            error_uri: None,
            state: None,
        }
    }

    pub fn access_denied() -> Self {
        Self::new(
            "access_denied".to_string(),
            Some("The user denied the request".to_string()),
        )
    }

    pub fn invalid_request(description: String) -> Self {
        Self::new("invalid_request".to_string(), Some(description))
    }

    pub fn server_error(description: String) -> Self {
        Self::new("server_error".to_string(), Some(description))
    }
}

/// PKCE code challenge and verifier pair
#[derive(Debug, Clone)]
pub struct PkcePair {
    pub code_verifier: String,
    pub code_challenge: String,
}

impl PkcePair {
    /// Generate PKCE pair using S256 method
    pub fn generate() -> Self {
        use rand::{distributions::Alphanumeric, Rng};
        use sha2::{Sha256, Digest};
        use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};

        // Generate code verifier (43-128 characters)
        let code_verifier: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(128)
            .map(char::from)
            .collect();

        // Generate code challenge using S256 method
        let mut hasher = Sha256::new();
        hasher.update(code_verifier.as_bytes());
        let code_challenge = URL_SAFE_NO_PAD.encode(hasher.finalize());

        Self {
            code_verifier,
            code_challenge,
        }
    }

    /// Verify code verifier against challenge
    pub fn verify(&self, verifier: &str) -> bool {
        use sha2::{Sha256, Digest};
        use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};

        let mut hasher = Sha256::new();
        hasher.update(verifier.as_bytes());
        let expected_challenge = URL_SAFE_NO_PAD.encode(hasher.finalize());
        
        expected_challenge == self.code_challenge
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pkce_generation_and_verification() {
        let pkce_pair = PkcePair::generate();
        
        // Should verify correctly
        assert!(pkce_pair.verify(&pkce_pair.code_verifier));
        
        // Should not verify with wrong verifier
        assert!(!pkce_pair.verify("wrong_verifier"));
    }

    #[test]
    fn test_oauth_state_expiry() {
        let state = OAuthState::new(
            "test_state".to_string(),
            "test_verifier".to_string(),
            "test_challenge".to_string(),
            None,
            -1, // Already expired
        );

        assert!(state.is_expired());
        assert!(!state.is_valid());
    }

    #[test]
    fn test_oauth_callback_params() {
        let success_params = OAuthCallbackParams {
            code: "auth_code".to_string(),
            state: "state_token".to_string(),
            scope: Some("email profile".to_string()),
            error: None,
            error_description: None,
        };

        assert!(success_params.is_success());
        assert!(!success_params.is_error());

        let error_params = OAuthCallbackParams {
            code: String::new(),
            state: "state_token".to_string(),
            scope: None,
            error: Some("access_denied".to_string()),
            error_description: Some("User denied access".to_string()),
        };

        assert!(!error_params.is_success());
        assert!(error_params.is_error());
        assert!(error_params.error_message().is_some());
    }
}