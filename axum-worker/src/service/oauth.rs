// //! OAuth Service - Complete Google OAuth 2.0 implementation with PKCE support

// use tracing::error;
// use uuid::Uuid;
// use worker::{console_log, Fetch, Method, Request, RequestInit};
// use std::sync::Mutex;
// use std::collections::HashMap;

// use crate::{
//     error::AppError,
//     dto::oauth::{
//     OAuthState, GoogleTokenResponse, GoogleUserInfo, PkcePair, 
//     OAuthProviderConfig
//     },
//     database::Database,
// };

// /// Static in-memory storage for OAuth states (fallback for local development)
// use std::sync::LazyLock;
// static OAUTH_STATES: LazyLock<Mutex<HashMap<String, OAuthState>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

// /// OAuth service for managing Google OAuth 2.0 flow with PKCE
// #[derive(Clone)]
// pub struct OAuthService {
//     pub(crate) config: OAuthProviderConfig,
//     pub(crate) database: Database,
// }

// impl OAuthService {
//     /// Create new OAuth service with Google configuration
//     pub fn new_google(
//         client_id: String,
//         client_secret: String,
//         redirect_uri: String,
//         database: Database,
//     ) -> Self {
//         let config = OAuthProviderConfig::google_config(client_id, client_secret, redirect_uri);
        
//         Self {
//             config,
//             database,
//         }
//     }

//     /// Create OAuth service with custom provider config
//     pub fn new(config: OAuthProviderConfig, database: Database) -> Self {
//         Self {
//             config,
//             database,
//         }
//     }

//     /// Generate PKCE pair using S256 method
//     pub fn generate_pkce_pair() -> PkcePair {
//         PkcePair::generate()
//     }

//     /// Create authorization URL with PKCE and state parameter
//     pub async fn create_authorization_url(
//         &self,
//         redirect_after: Option<String>,
//     ) -> Result<(String, String), AppError> {
//         // Generate PKCE pair
//         let pkce_pair = Self::generate_pkce_pair();
        
//         // Generate state token for CSRF protection
//         let state_token = Uuid::new_v4().to_string();
        
//         // Create OAuth state
//         let oauth_state = OAuthState::new(
//             state_token.clone(),
//             pkce_pair.code_verifier.clone(),
//             pkce_pair.code_challenge.clone(),
//             redirect_after,
//             300, // 5 minutes TTL
//         );
        
//         // Store state in database
//         self.store_oauth_state(&oauth_state).await?;
        
//         // Build authorization URL
//         let mut auth_url = format!(
//             "{}?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}&code_challenge={}&code_challenge_method={}",
//             self.config.auth_url,
//             urlencoding::encode(&self.config.client_id),
//             urlencoding::encode(&self.config.redirect_uri),
//             urlencoding::encode(&self.config.get_scope_string()),
//             urlencoding::encode(&state_token),
//             urlencoding::encode(&pkce_pair.code_challenge),
//             urlencoding::encode(&self.config.code_challenge_method),
//         );
        
//         // Add access_type for refresh token
//         if self.config.is_google() {
//             auth_url.push_str("&access_type=offline&prompt=consent");
//         }
        
//         console_log!("OAuth authorization URL created for state: {}", state_token);
        
//         Ok((auth_url, state_token))
//     }

//     /// Exchange authorization code for tokens
//     pub async fn exchange_code_for_tokens(
//         &self,
//         code: &str,
//         state: &str,
//     ) -> Result<GoogleTokenResponse, AppError> {
//         console_log!("Exchanging authorization code for tokens, state: {}", state);
        
//         // Retrieve and validate OAuth state using database
//         let oauth_state = self.validate_and_consume_state(state).await?;
        
//         // Prepare token exchange request
//         let token_params = vec![
//             ("client_id", self.config.client_id.as_str()),
//             ("client_secret", self.config.client_secret.as_str()),
//             ("code", code),
//             ("grant_type", "authorization_code"),
//             ("redirect_uri", self.config.redirect_uri.as_str()),
//             ("code_verifier", oauth_state.code_verifier.as_str()),
//         ];
        
//         let form_data = token_params
//             .iter()
//             .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
//             .collect::<Vec<_>>()
//             .join("&");
        
//         // Make token exchange request using worker::Fetch
//         let mut request_init = RequestInit::new();
//         request_init.method = Method::Post;
        
//     let headers = worker::Headers::new();
//         headers.set("Content-Type", "application/x-www-form-urlencoded")
//             .map_err(|e| AppError::ExternalServiceError(format!("Failed to set header: {:?}", e)))?;
//         headers.set("Accept", "application/json")
//             .map_err(|e| AppError::ExternalServiceError(format!("Failed to set header: {:?}", e)))?;
//         request_init.headers = headers;
        
//         request_init.body = Some(form_data.into());
        
//         let request = Request::new_with_init(&self.config.token_url, &request_init)
//             .map_err(|e| AppError::ExternalServiceError(format!("Failed to create token request: {:?}", e)))?;
        
//         let mut response = Fetch::Request(request).send().await
//             .map_err(|e| AppError::ExternalServiceError(format!("Token exchange request failed: {:?}", e)))?;
        
//         if response.status_code() != 200 {
//             let error_text = response.text().await.unwrap_or_default();
//             error!("Token exchange failed with status {}: {}", response.status_code(), error_text);
//             return Err(AppError::ExternalServiceError(format!(
//                 "Token exchange failed with status {}", response.status_code()
//             )));
//         }
        
//         let token_response: GoogleTokenResponse = response.json().await
//             .map_err(|e| AppError::ExternalServiceError(format!("Failed to parse token response: {:?}", e)))?;
        
//         console_log!("Successfully exchanged authorization code for tokens");
        
//         Ok(token_response)
//     }

//     /// Refresh access token using refresh token
//     pub async fn refresh_access_token(
//         &self,
//         refresh_token: &str,
//     ) -> Result<GoogleTokenResponse, AppError> {
//         console_log!("Refreshing access token");
        
//         let token_params = vec![
//             ("client_id", self.config.client_id.as_str()),
//             ("client_secret", self.config.client_secret.as_str()),
//             ("refresh_token", refresh_token),
//             ("grant_type", "refresh_token"),
//         ];
        
//         let form_data = token_params
//             .iter()
//             .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
//             .collect::<Vec<_>>()
//             .join("&");
        
//         // Make token refresh request using worker::Fetch
//         let mut request_init = RequestInit::new();
//         request_init.method = Method::Post;
        
//     let headers = worker::Headers::new();
//         headers.set("Content-Type", "application/x-www-form-urlencoded")
//             .map_err(|e| AppError::ExternalServiceError(format!("Failed to set header: {:?}", e)))?;
//         headers.set("Accept", "application/json")
//             .map_err(|e| AppError::ExternalServiceError(format!("Failed to set header: {:?}", e)))?;
//         request_init.headers = headers;
        
//         request_init.body = Some(form_data.into());
        
//         let request = Request::new_with_init(&self.config.token_url, &request_init)
//             .map_err(|e| AppError::ExternalServiceError(format!("Failed to create refresh request: {:?}", e)))?;
        
//         let mut response = Fetch::Request(request).send().await
//             .map_err(|e| AppError::ExternalServiceError(format!("Token refresh request failed: {:?}", e)))?;
        
//         if response.status_code() != 200 {
//             let error_text = response.text().await.unwrap_or_default();
//             error!("Token refresh failed with status {}: {}", response.status_code(), error_text);
//             return Err(AppError::ExternalServiceError(format!(
//                 "Token refresh failed with status {}", response.status_code()
//             )));
//         }
        
//         let token_response: GoogleTokenResponse = response.json().await
//             .map_err(|e| AppError::ExternalServiceError(format!("Failed to parse refresh response: {:?}", e)))?;
        
//         console_log!("Successfully refreshed access token");
        
//         Ok(token_response)
//     }

//     /// Get user info from OAuth provider
//     pub async fn get_user_info(
//         &self,
//         access_token: &str,
//     ) -> Result<GoogleUserInfo, AppError> {
//         console_log!("Fetching user info from OAuth provider");
        
//         // Make userinfo request using worker::Fetch
//         let mut request_init = RequestInit::new();
//         request_init.method = Method::Get;
        
//     let headers = worker::Headers::new();
//         headers.set("Authorization", &format!("Bearer {}", access_token))
//             .map_err(|e| AppError::ExternalServiceError(format!("Failed to set header: {:?}", e)))?;
//         headers.set("Accept", "application/json")
//             .map_err(|e| AppError::ExternalServiceError(format!("Failed to set header: {:?}", e)))?;
//         request_init.headers = headers;
        
//         let request = Request::new_with_init(&self.config.userinfo_url, &request_init)
//             .map_err(|e| AppError::ExternalServiceError(format!("Failed to create userinfo request: {:?}", e)))?;
        
//         let mut response = Fetch::Request(request).send().await
//             .map_err(|e| AppError::ExternalServiceError(format!("Userinfo request failed: {:?}", e)))?;
        
//         if response.status_code() != 200 {
//             let error_text = response.text().await.unwrap_or_default();
//             error!("Userinfo request failed with status {}: {}", response.status_code(), error_text);
//             return Err(AppError::ExternalServiceError(format!(
//                 "Userinfo request failed with status {}", response.status_code()
//             )));
//         }
        
//         let user_info: GoogleUserInfo = response.json().await
//             .map_err(|e| AppError::ExternalServiceError(format!("Failed to parse userinfo response: {:?}", e)))?;
        
//         console_log!("Successfully fetched user info for: {}", user_info.email);
        
//         Ok(user_info)
//     }

//     /// Validate OAuth state parameter (read-only validation)
//     pub async fn validate_state(&self, state: &str) -> Result<OAuthState, AppError> {
//         self.get_oauth_state(state).await
//     }

//     /// Validate and consume OAuth state parameter (removes it from store)
//     pub async fn validate_and_consume_state(&self, state: &str) -> Result<OAuthState, AppError> {
//         let oauth_state = self.get_oauth_state(state).await?;
//         self.remove_oauth_state(state).await?;
//         Ok(oauth_state)
//     }

//     /// Cleanup expired OAuth states
//     pub async fn cleanup_expired_states(&self) -> usize {
//         match self.database.execute(
//             "DELETE FROM oauth_states WHERE expires_at < NOW()",
//             &[]
//         ).await {
//             Ok(affected_rows) => {
//                 if affected_rows > 0 {
//                     console_log!("Cleaned up {} expired OAuth states", affected_rows);
//                 }
//                 affected_rows as usize
//             }
//             Err(e) => {
//                 error!("Failed to cleanup expired OAuth states: {}", e);
//                 0
//             }
//         }
//     }

//     /// Get provider configuration
//     pub fn get_provider_config(&self) -> &OAuthProviderConfig {
//         &self.config
//     }

//     /// Check if provider is Google
//     pub fn is_google_provider(&self) -> bool {
//         self.config.is_google()
//     }

//     /// Generate state token
//     pub fn generate_state_token() -> String {
//         Uuid::new_v4().to_string()
//     }

//     /// Verify PKCE code challenge
//     pub fn verify_pkce_challenge(
//         code_verifier: &str,
//         code_challenge: &str,
//     ) -> bool {
//         let pkce_pair = PkcePair {
//             code_verifier: code_verifier.to_string(),
//             code_challenge: code_challenge.to_string(),
//         };
        
//         pkce_pair.verify(code_verifier)
//     }

//     /// Store OAuth state in database (with fallback for local development)
//     async fn store_oauth_state(&self, oauth_state: &OAuthState) -> Result<(), AppError> {
//         // Try database first, fallback to in-memory for local development
//         match self.database.execute(
//             "INSERT INTO oauth_states (state_token, code_verifier, code_challenge, redirect_after, expires_at) 
//              VALUES ($1, $2, $3, $4, $5)",
//             &[
//                 &oauth_state.state,
//                 &oauth_state.code_verifier,
//                 &oauth_state.code_challenge,
//                 &oauth_state.redirect_after_login.as_ref(),
//                 &oauth_state.expires_at,
//             ]
//         ).await {
//             Ok(_) => {
//                 console_log!("🔐 OAuth state stored in database: {}", oauth_state.state);
//             },
//             Err(db_error) => {
//                 console_log!("🔐 Database connection failed (local dev), using memory fallback: {}", db_error);
//                 // Store in static memory for local development - LIVE DATA preserved
//                 OAUTH_STATES.lock().unwrap().insert(
//                     oauth_state.state.clone(),
//                     oauth_state.clone()
//                 );
//                 console_log!("🔐 OAuth state stored in memory fallback: {}", oauth_state.state);
//             }
//         }
        
//         console_log!("🔐 LIVE DATA: state={}, verifier={}, challenge={}", 
//                     oauth_state.state, 
//                     oauth_state.code_verifier,
//                     oauth_state.code_challenge);
//         Ok(())
//     }

//     /// Get OAuth state from database (with fallback for local development)
//     async fn get_oauth_state(&self, state_token: &str) -> Result<OAuthState, AppError> {
//         // Try database first, fallback to in-memory for local development
//         match self.database.query_one(
//             "SELECT state_token, code_verifier, code_challenge, redirect_after, expires_at, created_at 
//              FROM oauth_states WHERE state_token = $1",
//             &[&state_token]
//         ).await {
//             Ok(row) => {
//                 let oauth_state = OAuthState {
//                     state: row.get("state_token"),
//                     code_verifier: row.get("code_verifier"),
//                     code_challenge: row.get("code_challenge"),
//                     redirect_after_login: row.get("redirect_after"),
//                     expires_at: row.get("expires_at"),
//                     created_at: row.get("created_at"),
//                 };

//                 if !oauth_state.is_valid() {
//                     let _ = self.remove_oauth_state(state_token).await;
//                     return Err(AppError::AuthenticationError("OAuth state expired".to_string()));
//                 }

//                 console_log!("🔐 OAuth state retrieved from database: {}", state_token);
//                 Ok(oauth_state)
//             },
//             Err(_) => {
//                 // Fallback to memory storage for local development
//                 console_log!("🔐 Database connection failed, checking memory fallback: {}", state_token);
                
//                 let oauth_state = {
//                     let states = OAUTH_STATES.lock().unwrap();
//                     states.get(state_token).cloned()
//                 };
                
//                 if let Some(oauth_state) = oauth_state {
//                     if !oauth_state.is_valid() {
//                         let _ = self.remove_oauth_state(state_token).await;
//                         return Err(AppError::AuthenticationError("OAuth state expired".to_string()));
//                     }
                    
//                     console_log!("🔐 OAuth state retrieved from memory fallback: {}", state_token);
//                     Ok(oauth_state)
//                 } else {
//                     error!("OAuth state not found in database or memory: {}", state_token);
//                     Err(AppError::AuthenticationError("Invalid OAuth state".to_string()))
//                 }
//             }
//         }
//     }

//     /// Remove OAuth state from database (with fallback for local development)
//     async fn remove_oauth_state(&self, state_token: &str) -> Result<(), AppError> {
//         // Try database first, then remove from memory fallback
//         match self.database.execute(
//             "DELETE FROM oauth_states WHERE state_token = $1",
//             &[&state_token]
//         ).await {
//             Ok(affected_rows) => {
//                 if affected_rows == 0 {
//                     console_log!("🔐 OAuth state not found in database for removal: {}", state_token);
//                 } else {
//                     console_log!("🔐 OAuth state removed from database: {}", state_token);
//                 }
//             },
//             Err(_) => {
//                 // Fallback to memory storage removal
//                 console_log!("🔐 Database connection failed, removing from memory fallback: {}", state_token);
//                 let mut states = OAUTH_STATES.lock().unwrap();
//                 if states.remove(state_token).is_some() {
//                     console_log!("🔐 OAuth state removed from memory fallback: {}", state_token);
//                 } else {
//                     console_log!("🔐 OAuth state not found in memory for removal: {}", state_token);
//                 }
//             }
//         }
        
//         Ok(())
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[tokio::test]
//     async fn test_oauth_service_creation() {
//         let service = OAuthService::new_google(
//             "test_client_id".to_string(),
//             "test_client_secret".to_string(),
//             "http://localhost:8787/api/auth/oauth/callback".to_string(),
//         );

//         assert!(service.is_google_provider());
//         assert_eq!(service.config.client_id, "test_client_id");
//     }

//     #[tokio::test]
//     async fn test_pkce_generation() {
//         let pkce_pair = OAuthService::generate_pkce_pair();
        
//         assert!(!pkce_pair.code_verifier.is_empty());
//         assert!(!pkce_pair.code_challenge.is_empty());
//         assert!(pkce_pair.verify(&pkce_pair.code_verifier));
//     }

//     #[tokio::test]
//     async fn test_state_management() {
//         let service = OAuthService::new_google(
//             "test_client_id".to_string(),
//             "test_client_secret".to_string(),
//             "http://localhost:8787/api/auth/oauth/callback".to_string(),
//         );

//         let (url, state_token) = service.create_authorization_url(None).await.unwrap();
        
//         assert!(url.contains(&state_token));
//         assert!(url.contains("code_challenge="));
//         assert!(url.contains("code_challenge_method=S256"));
        
//         // Validate state exists
//         let state = service.validate_state(&state_token).await.unwrap();
//         assert_eq!(state.state, state_token);
//     }

//     #[test]
//     fn test_pkce_verification() {
//         let pkce_pair = PkcePair::generate();
        
//         assert!(OAuthService::verify_pkce_challenge(
//             &pkce_pair.code_verifier,
//             &pkce_pair.code_challenge
//         ));
        
//         assert!(!OAuthService::verify_pkce_challenge(
//             "wrong_verifier",
//             &pkce_pair.code_challenge
//         ));
//     }
// }