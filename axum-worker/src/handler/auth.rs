//! Improved Auth Handler - Uses simplified OAuth service aligned with database schema
/// OAuth handler that receives tokens from frontend
/// Frontend handles Google API calls to avoid Send trait issues in WASM

use axum::{
    http::{StatusCode, HeaderMap, HeaderValue},
    extract::{Query, State},
    response::{IntoResponse, Redirect, Json},
};
use serde::{Deserialize, Serialize};
use tracing::error;
use worker::console_log;

use crate::{
    utils::error::AppError,
    state::AppState,
    dto::oauth::OAuthCallbackParams,
    service::{session, auth::AuthService},
};
use std::sync::Arc;

/// OAuth login query parameters
#[derive(Deserialize)]
pub struct OAuthLoginParams {
    pub redirect_after: Option<String>,
}

/// OAuth submission from frontend (includes tokens and user info)
#[derive(Deserialize)]
pub struct FrontendOAuthSubmission {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: i64,
    pub state: String,
    pub code: Option<String>,
    // Space-delimited scopes string provided by frontend (Google returns 'scope' in token response)
    // Example: "openid email profile https://www.googleapis.com/auth/gmail.readonly"
    pub scope: Option<String>,
    pub user_info: UserInfo,
}

/// User info from Google
#[derive(Deserialize)]
pub struct UserInfo {
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

/// Response to frontend
#[derive(Serialize)]
pub struct OAuthResponse {
    pub success: bool,
    pub session_id: String,
    pub user_email: String,
    pub expires_at: String,
}


/// API response structure that matches frontend expectations
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

/// User structure that matches frontend User type
#[derive(Serialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub picture: Option<String>,
}

/// Development-only: token inspection response (non-generic to avoid type mismatch issues)
#[derive(Serialize)]
pub struct DevTokenInfoApiResponse {
    pub success: bool,
    pub data: Option<crate::repository::auth::DevGoogleTokenInfo>,
    pub message: Option<String>,
}

/// Improved OAuth login using simplified service (no database dependency for state)
pub async fn oauth_login_improved(
    Query(params): Query<OAuthLoginParams>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    console_log!("🔐 IMPROVED: OAuth login initiated with simplified service");

    // Use simplified OAuth service (no database dependency for PKCE state)
    let oauth = crate::service::oauth::SimplifiedOAuthService::new(
        state.google_oauth_config.client_id.clone(),
        state.google_oauth_config.client_secret.clone(),
        state.google_oauth_config.redirect_uri.clone(),
    );
    let (auth_url, state_token) = oauth
        .create_authorization_url(params.redirect_after)
        .await
        .map_err(|e| {
            error!("Failed to create OAuth authorization URL: {}", e);
            AppError::ExternalServiceError("Failed to initiate OAuth flow".to_string())
        })?;

    console_log!("🔐 IMPROVED: Redirecting to OAuth provider with state: {}", state_token);

    Ok(Redirect::temporary(&auth_url))
}

/// Get current user from session - GET /api/auth/user
/// This endpoint validates the session and returns user data in the format expected by the frontend
#[axum::debug_handler]
pub async fn get_current_user_improved(
    State(state): State<AppState>,
    req: axum::http::Request<axum::body::Body>,
) -> Result<impl IntoResponse, AppError> {
    
    let (parts, _body) = req.into_parts();
    
    // Extract session token from cookies
    let session_token = session::extract_session_token_from_cookies(&parts)
        .ok_or_else(|| AppError::AuthenticationError("No session found".to_string()))?;
    
    console_log!("🔐 IMPROVED: Validating session token for /api/auth/user");
    
    // Use service to validate session token and get user info
    let auth_service = AuthService::new(Arc::clone(&state.database));
    match auth_service.get_current_user(&session_token).await {
        Ok(user) => {
            let response = ApiResponse {
                success: true,
                data: Some(user),
                message: Some("Session valid".to_string()),
            };
            console_log!("🔐 IMPROVED: Session validation successful, returning user data");
            Ok(Json(response))
        }
        Err(e) => {
            console_log!("🔐 IMPROVED: Session validation failed: {}", e);
            let response = ApiResponse::<User> {
                success: false,
                data: None,
                message: Some(format!("Session validation failed: {}", e)),
            };
            Ok(Json(response))
        }
    }
}


/// POST /api/auth/logout
/// Clears the session cookie and invalidates the corresponding session/token in the database
pub async fn logout(
    State(state): State<AppState>,
    req: axum::http::Request<axum::body::Body>,
) -> Result<impl IntoResponse, AppError> {
    let (parts, _body) = req.into_parts();

    // Attempt to extract and parse the session cookie: format is user_id:token_id
    let maybe_cookie = session::extract_session_token_from_cookies(&parts);

    if let Some(session_token) = maybe_cookie {
        // Use service to handle logout
        let auth_service = AuthService::new(Arc::clone(&state.database));
        let _ = auth_service.logout(Some(session_token)).await;
    }

    // Always clear the cookie in the response
    let mut headers = HeaderMap::new();
    let secure = state.config.environment.to_lowercase() != "development";
    let cookie_value = if secure {
        // Secure attribute only outside development
        "session_id=; Path=/; Max-Age=0; SameSite=Lax; Secure"
    } else {
        "session_id=; Path=/; Max-Age=0; SameSite=Lax"
    };
    if let Ok(hv) = HeaderValue::from_str(cookie_value) {
        headers.insert("Set-Cookie", hv);
    }

    let resp = ApiResponse::<()> {
        success: true,
        data: None,
        message: Some("Logged out successfully".to_string()),
    };

    Ok((StatusCode::OK, headers, Json(resp)))
}

/// Redirect OAuth callback to frontend for handling
pub async fn oauth_callback_redirect(
    Query(params): Query<OAuthCallbackParams>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    console_log!("🔐 REDIRECT: OAuth callback received, redirecting to frontend");
    
    // Build frontend callback URL with all OAuth parameters
    // Always use configured base_url (defaults to http://localhost:8787)
    let frontend_url = &state.config.base_url;
    
    // Construct the redirect URL with OAuth parameters
    let mut redirect_url = format!("{}/auth/callback?", frontend_url.trim_end_matches('/'));
    
    // Add authorization code if present
    if !params.code.is_empty() {
        redirect_url.push_str(&format!("code={}&", urlencoding::encode(&params.code)));
    }
    
    // Add state parameter
    redirect_url.push_str(&format!("state={}", urlencoding::encode(&params.state)));
    
    // Add error parameters if present
    if let Some(error) = &params.error {
        redirect_url.push_str(&format!("&error={}", urlencoding::encode(error)));
    }
    if let Some(error_description) = &params.error_description {
        redirect_url.push_str(&format!("&error_description={}", urlencoding::encode(error_description)));
    }
    
    console_log!("🔐 REDIRECT: Redirecting to frontend: {}", redirect_url);
    
    Ok(Redirect::temporary(&redirect_url))
}

/// Handle OAuth tokens from frontend (frontend already exchanged with Google)
pub async fn handle_frontend_oauth(
    State(state): State<AppState>,
    Json(data): Json<FrontendOAuthSubmission>,
) -> impl IntoResponse {
    let (tx, rx) = futures::channel::oneshot::channel();
    let db = Arc::clone(&state.database);
    wasm_bindgen_futures::spawn_local(async move {
        let auth_service = AuthService::new(db);
        let result = auth_service.handle_frontend_oauth(data).await;
        let _ = tx.send(result);
    });
    match rx.await.unwrap_or_else(|_| Err(AppError::Internal("channel closed".into()))) {
        Ok(response) => {
            // Parse session parts to build cookie header
            let mut headers = HeaderMap::new();
            if let [user_id, token_id] = &response.session_id.split(':').collect::<Vec<&str>>()[..] {
                let secure = state.config.environment.to_lowercase() != "development";
                let cookie_val = if secure {
                    format!("session_id={}:{}; Path=/; SameSite=Lax; Secure; Max-Age={}", user_id, token_id, 24*60*60)
                } else {
                    format!("session_id={}:{}; Path=/; SameSite=Lax; Max-Age={}", user_id, token_id, 24*60*60)
                };
                if let Ok(hv) = HeaderValue::from_str(&cookie_val) { headers.insert("Set-Cookie", hv); }
            }
            (StatusCode::OK, headers, Json(response)).into_response()
        }
        Err(e) => {
            // Log the error so the variable is used (removes unused variable warning)
            error!("🔐 OAUTH FRONTEND: failed to handle frontend OAuth submission: {}", e);
            let err_resp = OAuthResponse { success: false, session_id: String::new(), user_email: String::new(), expires_at: chrono::Utc::now().to_rfc3339() };
            (StatusCode::BAD_REQUEST, Json(err_resp)).into_response()
        }
    }
}

/// GET /api/auth/dev/google-token?email=foo@example.com (development only)
pub async fn dev_google_token_info(
    State(state): State<AppState>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<impl IntoResponse, AppError> {
    if state.config.environment.to_lowercase() != "development" {
        return Ok((StatusCode::FORBIDDEN, Json(DevTokenInfoApiResponse { success: false, data: None, message: Some("Not available in this environment".into()) })));
    }
    let email = params.get("email").cloned().ok_or_else(|| AppError::BadRequest("Missing email param".into()))?;
    let repo = crate::repository::auth::AuthRepository::new(Arc::clone(&state.database));
    let resp = match repo.dev_fetch_google_token_info(&email).await? {
        Some(info) => (StatusCode::OK, Json(DevTokenInfoApiResponse { success: true, data: Some(info), message: None })),
        None => (StatusCode::NOT_FOUND, Json(DevTokenInfoApiResponse { success: false, data: None, message: Some("No token info found".into()) })),
    };
    Ok(resp)
}

/// GET /api/auth/dev/gmail-axiom-code?email=foo@example.com (development only)
/// Attempts to fetch recent Axiom 2FA code using stored Gmail access token
pub async fn dev_gmail_axiom_code(
    State(state): State<AppState>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> impl IntoResponse {
    // Keep handler future Send by offloading non-Send worker::Fetch calls to spawn_local
    if state.config.environment.to_lowercase() != "development" {
        let resp = serde_json::json!({"success": false, "message": "Not available in this environment"});
        return (StatusCode::FORBIDDEN, Json(resp)).into_response();
    }
    let email = match params.get("email").cloned() {
        Some(e) => e,
        None => {
            let resp = serde_json::json!({"success": false, "message": "Missing email param"});
            return (StatusCode::BAD_REQUEST, Json(resp)).into_response();
        }
    };
    let db = Arc::clone(&state.database);
    let (tx, rx) = futures::channel::oneshot::channel();
    let email_clone = email.clone();
    wasm_bindgen_futures::spawn_local(async move {
        let repo = crate::repository::auth::AuthRepository::new(Arc::clone(&db));
        let result: Result<serde_json::Value, (StatusCode, String)> = async {
            let access_token = match repo.get_latest_google_access_token(&email_clone).await {
                Ok(Some(t)) => t,
                Ok(None) => return Err((StatusCode::NOT_FOUND, "No access token found".into())),
                Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {}", e))),
            };
            let gmail = crate::service::gmail::GmailService::new(access_token).await
                .map_err(|e| (StatusCode::BAD_GATEWAY, format!("Init Gmail failed: {:?}", e)))?;
            let code = gmail.get_axiom_2fa_code(&email_clone).await
                .map_err(|e| (StatusCode::BAD_GATEWAY, format!("Gmail API error: {:?}", e)))?;
            Ok(serde_json::json!({ "success": true, "email": email_clone, "axiom_code": code }))
        }.await;
        let _ = tx.send(result);
    });
    match rx.await {
        Ok(Ok(json)) => (StatusCode::OK, Json(json)).into_response(),
        Ok(Err((status, msg))) => {
            let json = serde_json::json!({"success": false, "message": msg});
            (status, Json(json)).into_response()
        }
        Err(_) => {
            let json = serde_json::json!({"success": false, "message": "Channel closed"});
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json)).into_response()
        }
    }
}

/// POST /api/auth/gmail/axiom-2fa
/// Frontend requests backend to pull latest Axiom Gmail 2FA code using stored access token.
#[derive(Deserialize)]
pub struct GmailAxiom2FARequest { pub user_email: String }
#[derive(Serialize)]
pub struct GmailAxiom2FAResponse { pub success: bool, pub message: Option<String>, pub otp_code: Option<String> }
pub async fn gmail_axiom_2fa(
    State(state): State<AppState>,
    Json(body): Json<GmailAxiom2FARequest>,
) -> impl IntoResponse {
    let email = body.user_email;
    let db = Arc::clone(&state.database);
    let (tx, rx) = futures::channel::oneshot::channel();
    wasm_bindgen_futures::spawn_local(async move {
        let repo = crate::repository::auth::AuthRepository::new(Arc::clone(&db));
        let result: Result<GmailAxiom2FAResponse, GmailAxiom2FAResponse> = async {
            let access_token = match repo.get_latest_google_access_token(&email).await {
                Ok(Some(t)) => t,
                Ok(None) => return Err(GmailAxiom2FAResponse { success: false, message: Some("No access token found".into()), otp_code: None }),
                Err(e) => return Err(GmailAxiom2FAResponse { success: false, message: Some(format!("DB error: {}", e)), otp_code: None }),
            };
            let gmail = match crate::service::gmail::GmailService::new(access_token).await {
                Ok(g) => g,
                Err(e) => return Err(GmailAxiom2FAResponse { success: false, message: Some(format!("Init Gmail failed: {:?}", e)), otp_code: None }),
            };
            match gmail.get_axiom_2fa_code(&email).await {
                Ok(Some(code)) => Ok(GmailAxiom2FAResponse { success: true, message: None, otp_code: Some(code) }),
                Ok(None) => Err(GmailAxiom2FAResponse { success: false, message: Some("No 2FA code found".into()), otp_code: None }),
                Err(e) => Err(GmailAxiom2FAResponse { success: false, message: Some(format!("Gmail API error: {:?}", e)), otp_code: None }),
            }
        }.await;
        let _ = tx.send(result);
    });
    match rx.await {
        Ok(Ok(resp)) => (StatusCode::OK, Json(resp)).into_response(),
        Ok(Err(err)) => {
            // Distinguish not found vs other errors for better frontend UX
            let status = if err.message.as_deref() == Some("No 2FA code found") { StatusCode::NOT_FOUND } else { StatusCode::BAD_REQUEST };
            (status, Json(err)).into_response()
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(GmailAxiom2FAResponse { success: false, message: Some("Channel closed".into()), otp_code: None })).into_response(),
    }
}
