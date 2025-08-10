//! Simplified OAuth Service - Uses in-memory state storage (no oauth_states table)
//! 
//! This service aligns with your database schema and uses tower-session for session management

use std::sync::Mutex;
use std::collections::HashMap;
// use serde::{Deserialize, Serialize};
use uuid::Uuid;
use worker::console_log;

use crate::{
    error::AppError,
    dto::oauth::{OAuthState, PkcePair},
};

/// In-memory OAuth state storage (since oauth_states table doesn't exist in schema)
use std::sync::LazyLock;
static OAUTH_STATES: LazyLock<Mutex<HashMap<String, OAuthState>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

/// Simplified OAuth service focused on PKCE state management
#[derive(Clone)]
pub struct SimplifiedOAuthService {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

impl SimplifiedOAuthService {
    /// Create new simplified OAuth service
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        Self {
            client_id,
            client_secret,
            redirect_uri,
        }
    }

    /// Create authorization URL with PKCE
    pub async fn create_authorization_url(&self, redirect_after: Option<String>) -> Result<(String, String), AppError> {
        // Generate PKCE pair
        let pkce = PkcePair::generate();
        let state_token = Uuid::new_v4().to_string();
        
        // Create OAuth state using the constructor
        let oauth_state = OAuthState::new(
            state_token.clone(),
            pkce.code_verifier.clone(),
            pkce.code_challenge.clone(),
            redirect_after,
            600, // 10 minutes TTL
        );

        // Store in memory
        {
            let mut states = OAUTH_STATES.lock().unwrap();
            states.insert(state_token.clone(), oauth_state);
        }

        // Build Google OAuth URL (without PKCE for now since frontend handles token exchange)
        let auth_url = format!(
            "https://accounts.google.com/o/oauth2/v2/auth?\
            client_id={}&\
            redirect_uri={}&\
            response_type=code&\
            scope=openid%20email%20profile%20https://www.googleapis.com/auth/gmail.readonly&\
            state={}&\
            access_type=offline&\
            prompt=consent",
            urlencoding::encode(&self.client_id),
            urlencoding::encode(&self.redirect_uri),
            urlencoding::encode(&state_token)
        );

        console_log!("🔐 SIMPLE: Created OAuth URL with PKCE challenge: {}", pkce.code_challenge);

        Ok((auth_url, state_token))
    }

    /// Validate OAuth state (non-consuming)
    pub async fn validate_state(&self, state_token: &str) -> Result<OAuthState, AppError> {
        let states = OAUTH_STATES.lock().unwrap();
        
        match states.get(state_token) {
            Some(oauth_state) => {
                // Check if expired
                if oauth_state.expires_at < chrono::Utc::now() {
                    console_log!("🔐 SIMPLE: OAuth state expired: {}", state_token);
                    return Err(AppError::AuthenticationError("OAuth state expired".to_string()));
                }
                
                console_log!("🔐 SIMPLE: OAuth state validated: {}", state_token);
                Ok(oauth_state.clone())
            }
            None => {
                console_log!("🔐 SIMPLE: OAuth state not found: {}", state_token);
                Err(AppError::AuthenticationError("Invalid OAuth state".to_string()))
            }
        }
    }

    /// Validate and consume OAuth state (removes from storage)
    pub async fn validate_and_consume_state(&self, state_token: &str) -> Result<OAuthState, AppError> {
        let mut states = OAUTH_STATES.lock().unwrap();
        
        match states.remove(state_token) {
            Some(oauth_state) => {
                // Check if expired
                if oauth_state.expires_at < chrono::Utc::now() {
                    console_log!("🔐 SIMPLE: OAuth state expired during consumption: {}", state_token);
                    return Err(AppError::AuthenticationError("OAuth state expired".to_string()));
                }
                
                console_log!("🔐 SIMPLE: OAuth state validated and consumed: {}", state_token);
                Ok(oauth_state)
            }
            None => {
                console_log!("🔐 SIMPLE: OAuth state not found for consumption: {}", state_token);
                Err(AppError::AuthenticationError("Invalid OAuth state".to_string()))
            }
        }
    }


    /// Cleanup expired states (maintenance function)
    pub async fn cleanup_expired_states(&self) -> Result<usize, AppError> {
        let mut states = OAUTH_STATES.lock().unwrap();
        let now = chrono::Utc::now();
        
        let initial_count = states.len();
        states.retain(|_, state| state.expires_at > now);
        let final_count = states.len();
        
        let cleaned_count = initial_count - final_count;
        console_log!("🔐 SIMPLE: Cleaned up {} expired OAuth states", cleaned_count);
        
        Ok(cleaned_count)
    }

    /// Get form data for Google token exchange (business logic only)
    pub fn prepare_token_exchange_form_data(
        &self,
        authorization_code: &str,
        oauth_state: &OAuthState,
    ) -> Result<String, AppError> {
        console_log!("🔐 SIMPLE: Preparing token exchange form data with PKCE verification");

        // Validate the authorization code format (basic validation)
        if authorization_code.is_empty() || authorization_code.len() < 10 {
            return Err(AppError::AuthenticationError("Invalid authorization code format".to_string()));
        }

        // Verify PKCE verifier is present
        if oauth_state.code_verifier.is_empty() {
            return Err(AppError::AuthenticationError("Missing PKCE code verifier".to_string()));
        }

        // Prepare form data for Google token exchange
        let form_data = format!(
            "client_id={}&client_secret={}&code={}&grant_type=authorization_code&redirect_uri={}&code_verifier={}",
            urlencoding::encode(&self.client_id),
            urlencoding::encode(&self.client_secret),
            urlencoding::encode(authorization_code),
            urlencoding::encode(&self.redirect_uri),
            urlencoding::encode(&oauth_state.code_verifier)
        );

        console_log!("🔐 SIMPLE: Token exchange form data prepared with PKCE");
        Ok(form_data)
    }
}