// //! Authentication Service - Business logic for user authentication and OAuth
// //! Phase 1: Simplified service without complex OAuth implementation

// use std::collections::HashMap;
// use uuid::Uuid;

// use crate::{
//     dto::auth::{GoogleTokenResponse, GoogleUserInfo, UserResponse},
//     repository::user::User, // Use repository User for Phase 1
//     error::{AppError, AuthError},
//     state::GoogleOAuthConfig,
// };

// /// Authentication service handling OAuth flows and user management
// pub struct AuthService {
//     oauth_config: GoogleOAuthConfig,
// }

// impl AuthService {
//     /// Create new authentication service
//     pub fn new(oauth_config: GoogleOAuthConfig) -> Self {
//         Self { oauth_config }
//     }

//     /// Generate Google OAuth authorization URL
//     pub fn generate_oauth_url(&self, state: Option<String>) -> String {
//         let mut params = vec![
//             ("client_id", self.oauth_config.client_id.as_str()),
//             ("redirect_uri", self.oauth_config.redirect_uri.as_str()),
//             ("response_type", "code"),
//             ("scope", "openid email profile"),
//             ("access_type", "offline"),
//             ("prompt", "consent"),
//         ];

//         if let Some(state_val) = state.as_ref() {
//             params.push(("state", state_val));
//         }

//         let query_string = params
//             .iter()
//             .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
//             .collect::<Vec<_>>()
//             .join("&");

//         format!("{}?{}", self.oauth_config.auth_url, query_string)
//     }

//     /// Exchange OAuth code for access token
//     pub async fn exchange_code_for_token(&self, code: &str) -> Result<GoogleTokenResponse, AppError> {
//         let mut params = HashMap::new();
//         params.insert("client_id", self.oauth_config.client_id.as_str());
//         params.insert("client_secret", self.oauth_config.client_secret.as_str());
//         params.insert("code", code);
//         params.insert("grant_type", "authorization_code");
//         params.insert("redirect_uri", self.oauth_config.redirect_uri.as_str());

//         // For WASM compatibility, we'll need to use worker::Fetch
//         // This is a simplified version - in production, you'd want proper error handling
//         let response = worker::Fetch::Request(worker::Request::new_with_init(
//             &self.oauth_config.token_url,
//             worker::RequestInit::new()
//                 .with_method(worker::Method::Post)
//                 .with_headers({
//                     let mut headers = worker::Headers::new();
//                     headers.set("Content-Type", "application/x-www-form-urlencoded")?;
//                     headers.set("Accept", "application/json")?;
//                     headers
//                 })
//                 .with_body(Some(
//                     params
//                         .iter()
//                         .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
//                         .collect::<Vec<_>>()
//                         .join("&")
//                         .into(),
//                 )),
//         )?)
//         .send()
//         .await
//         .map_err(|e| AppError::ExternalService(format!("Failed to exchange OAuth code: {:?}", e)))?;

//         if !response.ok() {
//             return Err(AppError::Auth(AuthError::OAuthError(
//                 "Failed to exchange code for token".to_string(),
//             )));
//         }

//         let token_response: GoogleTokenResponse = response
//             .json()
//             .await
//             .map_err(|e| AppError::ExternalService(format!("Failed to parse token response: {:?}", e)))?;

//         Ok(token_response)
//     }

//     /// Get user info from Google API using access token
//     pub async fn get_google_user_info(&self, access_token: &str) -> Result<GoogleUserInfo, AppError> {
//         let url = format!(
//             "https://www.googleapis.com/oauth2/v1/userinfo?access_token={}",
//             access_token
//         );

//         let response = worker::Fetch::Request(worker::Request::new_with_init(
//             &url,
//             worker::RequestInit::new()
//                 .with_method(worker::Method::Get)
//                 .with_headers({
//                     let mut headers = worker::Headers::new();
//                     headers.set("Accept", "application/json")?;
//                     headers
//                 }),
//         )?)
//         .send()
//         .await
//         .map_err(|e| AppError::ExternalService(format!("Failed to get user info: {:?}", e)))?;

//         if !response.ok() {
//             return Err(AppError::Auth(AuthError::OAuthError(
//                 "Failed to get user info from Google".to_string(),
//             )));
//         }

//         let user_info: GoogleUserInfo = response
//             .json()
//             .await
//             .map_err(|e| AppError::ExternalService(format!("Failed to parse user info: {:?}", e)))?;

//         Ok(user_info)
//     }

//     /// Create or update user from Google OAuth info
//     pub async fn create_or_update_oauth_user(&self, google_user: GoogleUserInfo) -> Result<User, AppError> {
//         // In a real implementation, you'd save this to a database
//         // For now, we'll create a new user entity
//         let user = User {
//             id: Uuid::new_v4().to_string(),
//             email: google_user.email,
//             name: google_user.given_name,
//             created_at: chrono::Utc::now(),
//         };

//         Ok(user)
//     }

//     /// Validate user credentials for traditional login
//     pub async fn validate_credentials(&self, email: &str, password: &str) -> Result<User, AppError> {
//         // In a real implementation, you'd query the database
//         // For now, this is a placeholder
//         Err(AppError::Auth(AuthError::InvalidCredentials))
//     }

//     /// Register new user with email/password
//     pub async fn register_user(
//         &self,
//         email: String,
//         _password: String,
//         first_name: Option<String>,
//         _last_name: Option<String>,
//     ) -> Result<User, AppError> {
//         // In a real implementation, you'd check if user exists and save to database
//         let user = User {
//             id: Uuid::new_v4().to_string(),
//             email,
//             name: first_name,
//             created_at: chrono::Utc::now(),
//         };

//         Ok(user)
//     }
// }

// /// Convert User entity to UserResponse DTO
// impl From<User> for UserResponse {
//     fn from(user: User) -> Self {
//         Self {
//             id: uuid::Uuid::parse_str(&user.id).unwrap_or_else(|_| uuid::Uuid::new_v4()),
//             email: user.email,
//             first_name: user.name.clone(),
//             last_name: None, // Repository User doesn't have separate last_name
//             is_oauth: false, // Repository User doesn't track OAuth status yet
//             created_at: user.created_at,
//         }
//     }
// }
