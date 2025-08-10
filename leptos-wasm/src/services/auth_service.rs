#![allow(dead_code)]
use crate::services::api_client::ApiClient;
use crate::types::auth::{ApiResponse, User, GoogleTokenResponse, GoogleUserInfo, OAuthTokenSubmission, OAuthTokenResponse};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::{window, Request, RequestInit, RequestMode, Headers};
use wasm_bindgen_futures::JsFuture;

/// Authentication service for managing user sessions and authentication
#[derive(Clone)]
pub struct AuthService {
    api_client: ApiClient,
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

impl AuthService {
    /// Create a new authentication service
    pub fn new() -> Self {
        Self {
            api_client: ApiClient::new(),
        }
    }

    /// Validate current session with the backend
    pub async fn validate_session(&self) -> Result<User, String> {
        match self.api_client.get::<ApiResponse<User>>("/auth/user").await {
            Ok(response) => {
                if response.success {
                    response.data.ok_or_else(|| "No user data in response".to_string())
                } else {
                    Err(response.message.unwrap_or_else(|| "Session validation failed".to_string()))
                }
            }
            Err(e) => Err(format!("Session validation request failed: {}", e)),
        }
    }

    /// Logout the current user
    pub async fn logout(&self) -> Result<(), String> {
        match self.api_client.post_empty::<ApiResponse<()>>("/auth/logout").await {
            Ok(response) => {
                if response.success {
                    Ok(())
                } else {
                    Err(response.message.unwrap_or_else(|| "Logout failed".to_string()))
                }
            }
            Err(e) => Err(format!("Logout request failed: {}", e)),
        }
    }

    /// Refresh the authentication token
    pub async fn refresh_token(&self) -> Result<(), String> {
        match self.api_client.post_empty::<ApiResponse<()>>("/auth/refresh").await {
            Ok(response) => {
                if response.success {
                    Ok(())
                } else {
                    Err(response.message.unwrap_or_else(|| "Token refresh failed".to_string()))
                }
            }
            Err(e) => Err(format!("Token refresh request failed: {}", e)),
        }
    }

    /// Initiate OAuth login by redirecting to the OAuth provider
    pub fn initiate_oauth_login(&self) -> Result<(), String> {
        let window = window().ok_or("Failed to get window object")?;
        
        // Redirect to backend OAuth endpoint - wrangler dev server on port 8787
        let auth_url = "http://127.0.0.1:8787/api/auth/oauth/login";
        
        window
            .location()
            .set_href(auth_url)
            .map_err(|_| "Failed to redirect to OAuth login".to_string())?;

        Ok(())
    }

    /// Get current user info from the API
    pub async fn get_current_user(&self) -> Result<User, String> {
        self.validate_session().await
    }

    /// Check if user is authenticated by attempting to validate session
    pub async fn is_authenticated(&self) -> bool {
        self.validate_session().await.is_ok()
    }

    /// Get user profile information
    pub async fn get_user_profile(&self) -> Result<User, String> {
        match self.api_client.get::<ApiResponse<User>>("/auth/profile").await {
            Ok(response) => {
                if response.success {
                    response.data.ok_or_else(|| "No profile data in response".to_string())
                } else {
                    Err(response.message.unwrap_or_else(|| "Failed to get profile".to_string()))
                }
            }
            Err(e) => Err(format!("Profile request failed: {}", e)),
        }
    }

    /// Update user profile
    pub async fn update_user_profile(&self, user_data: &User) -> Result<User, String> {
        match self.api_client.put::<User, ApiResponse<User>>("/auth/profile", user_data).await {
            Ok(response) => {
                if response.success {
                    response.data.ok_or_else(|| "No updated profile data in response".to_string())
                } else {
                    Err(response.message.unwrap_or_else(|| "Profile update failed".to_string()))
                }
            }
            Err(e) => Err(format!("Profile update request failed: {}", e)),
        }
    }

    /// Handle OAuth callback - exchange code with Google, then send to backend
    pub async fn handle_oauth_callback(&self, code: String, state: String) -> Result<OAuthTokenResponse, String> {
        log::info!("🔐 Frontend: ====== Starting OAuth Callback Processing ======");
        log::info!("🔐 Frontend: Authorization code received: {}", &code[..20.min(code.len())]);
        log::info!("🔐 Frontend: State parameter: {}", &state[..20.min(state.len())]);

        // Step 1: Exchange authorization code with Google for tokens
        log::info!("🔐 Frontend: Step 1 - Exchanging authorization code with Google");
        let token_response = self.exchange_code_for_tokens(&code, &state).await?;
        log::info!("🔐 Frontend: ✅ Token exchange successful");
        log::info!("🔐 Frontend: Access token: present={}, Refresh token: present={}", 
            !token_response.access_token.is_empty(), 
            token_response.refresh_token.is_some());
        
        // Step 2: Get user info from Google using the access token
        log::info!("🔐 Frontend: Step 2 - Getting user info from Google");
        let user_info = self.get_user_info_from_google(&token_response.access_token).await?;
        log::info!("🔐 Frontend: ✅ User info retrieved - Email: {}, ID: {}", 
            user_info.email, user_info.id);
        
        // Step 3: Send tokens and user info to backend
        log::info!("🔐 Frontend: Step 3 - Sending tokens and user info to backend");
        log::info!("🔐 Frontend: Payload - User: {}, Name: {:?}", 
            user_info.email, user_info.name);
        let submission = OAuthTokenSubmission {
            access_token: token_response.access_token,
            refresh_token: token_response.refresh_token,
            expires_in: token_response.expires_in,
            state,
            code: Some(code), // Still include the code for backend verification
            user_info, // Real user info from Google
        };

        let backend_response = self.submit_oauth_tokens(submission).await?;
        log::info!("🔐 Frontend: ✅ Successfully submitted OAuth data to backend");
        log::info!("🔐 Frontend: Backend response - Success: {}, Session: {}, User: {}", 
            backend_response.success, backend_response.session_id, backend_response.user_email);

        // Session cookie is set by the backend via Set-Cookie header
        log::info!("🔐 Frontend: Session cookie should be set by backend via Set-Cookie header");
        log::info!("🔐 Frontend: ====== OAuth Callback Processing Complete ======");

        Ok(backend_response)
    }

    /// Exchange authorization code for tokens with Google
    pub async fn exchange_code_for_tokens(&self, code: &str, _state: &str) -> Result<GoogleTokenResponse, String> {
        log::info!("🔐 Frontend: Exchanging code for tokens with Google");

        // Prepare token exchange request to Google (without PKCE)
        let token_params = format!(
            "client_id={}&client_secret={}&code={}&grant_type=authorization_code&redirect_uri={}",
            urlencoding::encode("1025434526562-aakomfir0hbmf8tu4a8tlrkeq4bmvgtc.apps.googleusercontent.com"),
            urlencoding::encode("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"), // This should be from env in production
            urlencoding::encode(code),
            urlencoding::encode("http://127.0.0.1:8787/api/auth/oauth/callback")
        );

        // Make direct fetch request to Google's token endpoint
        let window = web_sys::window().ok_or("No window object")?;
    let request_init = RequestInit::new();
        request_init.set_method("POST");
        request_init.set_mode(RequestMode::Cors);
        
        let headers = Headers::new().map_err(|_| "Failed to create headers")?;
        headers.set("Content-Type", "application/x-www-form-urlencoded").map_err(|_| "Failed to set content type")?;
        headers.set("Accept", "application/json").map_err(|_| "Failed to set accept header")?;
        request_init.set_headers(&headers);
        request_init.set_body(&wasm_bindgen::JsValue::from_str(&token_params));

        let request = Request::new_with_str_and_init(
            "https://oauth2.googleapis.com/token",
            &request_init
        ).map_err(|_| "Failed to create request")?;

        let response_promise = window.fetch_with_request(&request);
        let response = JsFuture::from(response_promise).await.map_err(|_| "Token request failed")?;
        let response: web_sys::Response = response.dyn_into().map_err(|_| "Invalid response type")?;

        if !response.ok() {
            return Err(format!("Google token request failed with status: {}", response.status()));
        }

        let json_promise = response.json().map_err(|_| "Failed to get JSON")?;
        let json_value = JsFuture::from(json_promise).await.map_err(|_| "Failed to parse JSON")?;
        
        // Log the raw token response to debug
        log::debug!("🔐 Frontend: Raw Google token response: {:?}", json_value);
        // Can't access JsValue properties directly, will get from deserialized response
        
        let token_response: GoogleTokenResponse = serde_wasm_bindgen::from_value(json_value)
            .map_err(|e| format!("Failed to deserialize token response: {:?}", e))?;
        
        log::info!("🔐 Frontend: Token exchange complete - expires in {} seconds", token_response.expires_in);

        Ok(token_response)
    }

    /// Get user info from Google using access token
    pub async fn get_user_info_from_google(&self, access_token: &str) -> Result<GoogleUserInfo, String> {
        log::info!("🔐 Frontend: Getting user info from Google");

        let window = web_sys::window().ok_or("No window object")?;
    let request_init = RequestInit::new();
        request_init.set_method("GET");
        request_init.set_mode(RequestMode::Cors);
        
        let headers = Headers::new().map_err(|_| "Failed to create headers")?;
        headers.set("Authorization", &format!("Bearer {}", access_token)).map_err(|_| "Failed to set auth header")?;
        headers.set("Accept", "application/json").map_err(|_| "Failed to set accept header")?;
        request_init.set_headers(&headers);

        let request = Request::new_with_str_and_init(
            "https://www.googleapis.com/oauth2/v2/userinfo",
            &request_init
        ).map_err(|_| "Failed to create request")?;

        let response_promise = window.fetch_with_request(&request);
        let response = JsFuture::from(response_promise).await.map_err(|_| "User info request failed")?;
        let response: web_sys::Response = response.dyn_into().map_err(|_| "Invalid response type")?;

        if !response.ok() {
            return Err(format!("Google user info request failed with status: {}", response.status()));
        }

        let json_promise = response.json().map_err(|_| "Failed to get JSON")?;
        let json_value = JsFuture::from(json_promise).await.map_err(|_| "Failed to parse JSON")?;
        
        // Log the raw response to see what fields Google is returning
        log::debug!("🔐 Frontend: Raw Google userinfo response: {:?}", json_value);
        // Can't access JsValue properties directly, will log from deserialized response
        
        let user_info: GoogleUserInfo = serde_wasm_bindgen::from_value(json_value)
            .map_err(|e| format!("Failed to deserialize user info: {:?}", e))?;
        
        log::info!("🔐 Frontend: User info retrieved - email: {}, name: {:?}, verified: {:?}",
            user_info.email, user_info.name, user_info.email_verified);

        Ok(user_info)
    }

    /// Submit OAuth tokens and user info to backend
    pub async fn submit_oauth_tokens(&self, submission: OAuthTokenSubmission) -> Result<OAuthTokenResponse, String> {
        log::info!("🔐 Frontend: Submitting OAuth data to backend /api/auth/oauth/token");
        log::info!("🔐 Frontend: Submission contains - user: {}, state: {}", 
            submission.user_info.email, &submission.state[..20.min(submission.state.len())]);

        match self.api_client.post::<OAuthTokenSubmission, OAuthTokenResponse>("/auth/oauth/token", &submission).await {
            Ok(response) => {
                log::info!("🔐 Frontend: Backend accepted OAuth submission");
                Ok(response)
            },
            Err(e) => {
                log::error!("🔐 Frontend: Backend rejected OAuth submission: {}", e);
                Err(format!("Failed to submit OAuth tokens to backend: {}", e))
            }
        }
    }
}

impl Default for AuthService {
    fn default() -> Self {
        Self::new()
    }
}

// Trigger rebuild
