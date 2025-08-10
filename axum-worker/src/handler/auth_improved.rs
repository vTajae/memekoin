//! Improved Auth Handler - Uses simplified OAuth service aligned with database schema

use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect, Json},
};
use serde::{Deserialize, Serialize};
use tracing::error;
use uuid::Uuid;
use worker::console_log;

use crate::{
    error::AppError,
    state::AppState,
    dto::oauth::OAuthCallbackParams,
};

/// OAuth login query parameters
#[derive(Deserialize)]
pub struct OAuthLoginParams {
    pub redirect_after: Option<String>,
}

/// Improved OAuth login using simplified service (no database dependency for state)
pub async fn oauth_login_improved(
    Query(params): Query<OAuthLoginParams>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    console_log!("🔐 IMPROVED: OAuth login initiated with simplified service");

    // Use simplified OAuth service (no database dependency for PKCE state)
    let oauth = crate::service::oauth_simple::SimplifiedOAuthService::new(
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

// COMMENTED OUT: Unused oauth_callback_improved function
// This was replaced with the working simulate_oauth_callback function
/*
/// Improved OAuth callback - completes the OAuth flow server-side with comprehensive logging
#[axum::debug_handler]
pub async fn oauth_callback_improved(
    Query(params): Query<OAuthCallbackParams>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    console_log!("🔐 CALLBACK: Received - state: {}, code present: {}", params.state, !params.code.is_empty());

    // Handle OAuth errors first
    if params.is_error() {
        let error_msg = params.error_message().unwrap_or("OAuth authorization failed".to_string());
        error!("OAuth callback error: {}", error_msg);
        let redirect_url = format!(
            "{}/auth/callback?error={}",
            state.config.base_url.trim_end_matches('/'),
            urlencoding::encode(&error_msg)
        );
        console_log!("🔐 CALLBACK: Redirecting with error to {}", redirect_url);
        return Ok(Redirect::temporary(&redirect_url).into_response());
    }

    // Validate the OAuth state parameter and CONSUME it to prevent replay
    let oauth = crate::service::oauth_simple::SimplifiedOAuthService::new(
        state.google_oauth_config.client_id.clone(),
        state.google_oauth_config.client_secret.clone(),
        state.google_oauth_config.redirect_uri.clone(),
    );
    let oauth_state = oauth
        .validate_and_consume_state(&params.state)
        .await
        .map_err(|e| {
            error!("OAuth state validation failed: {}", e);
            AppError::AuthenticationError("Invalid OAuth state".to_string())
        })?;

    console_log!("🔐 CALLBACK: State validated; preparing token exchange (PKCE present: {})", !oauth_state.code_verifier.is_empty());

    // Prepare form data for token exchange (do NOT log secrets)
    let form_data = oauth.prepare_token_exchange_form_data(&params.code, &oauth_state)?;
    console_log!("🔐 CALLBACK: Token exchange form prepared (client_id, redirect_uri, code_verifier set)");

    // Perform token exchange with Google
    use worker::{Fetch, Request, RequestInit, Method, Headers};
    let mut init = RequestInit::new();
    init.method = Method::Post;
    let headers = Headers::new();
    headers.set("Content-Type", "application/x-www-form-urlencoded").map_err(|e| AppError::ExternalServiceError(format!("Header set failed: {:?}", e)))?;
    headers.set("Accept", "application/json").map_err(|e| AppError::ExternalServiceError(format!("Header set failed: {:?}", e)))?;
    init.headers = headers;
    init.body = Some(form_data.clone().into());

    console_log!("🔐 CALLBACK: Exchanging authorization code with Google token endpoint");
    let request = Request::new_with_init(&state.google_oauth_config.token_url, &init)
        .map_err(|e| AppError::ExternalServiceError(format!("Failed to create token request: {:?}", e)))?;
    let mut resp = Fetch::Request(request).send().await
        .map_err(|e| AppError::ExternalServiceError(format!("Token exchange request failed: {:?}", e)))?;

    if resp.status_code() != 200 {
        let error_text = resp.text().await.unwrap_or_default();
        error!("Token exchange failed ({}): {}", resp.status_code(), error_text);
        let redirect_url = format!(
            "{}/auth/callback?error={}",
            state.config.base_url.trim_end_matches('/'),
            urlencoding::encode(&format!("Google token request failed with status: {}", resp.status_code()))
        );
        console_log!("🔐 CALLBACK: Redirecting to error page: {}", redirect_url);
        return Ok(Redirect::temporary(&redirect_url).into_response());
    }

    #[derive(Deserialize, Debug, Clone)]
    struct GoogleTokenResponseFull {
        access_token: String,
        refresh_token: Option<String>,
        expires_in: i64,
        token_type: String,
        id_token: Option<String>,
    }

    let token_response = resp.json::<GoogleTokenResponseFull>().await
        .map_err(|e| AppError::ExternalServiceError(format!("Failed to parse token response: {:?}", e)))?;
    console_log!("🔐 CALLBACK: Token exchange succeeded; expires_in={} token_type={}", token_response.expires_in, token_response.token_type);

    // Extract user info from ID token if possible; otherwise fetch userinfo endpoint
    let mut actual_email = String::new();
    let mut actual_sub = String::new();
    let mut given_name: Option<String> = None;
    let mut family_name: Option<String> = None;
    let mut display_name: Option<String> = None;
    let mut picture: Option<String> = None;

    if let Some(idt) = &token_response.id_token {
        // Parse JWT payload
        let parts: Vec<&str> = idt.split('.').collect();
        if parts.len() == 3 {
            if let Ok(payload_bytes) = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(parts[1]) {
                if let Ok(payload_str) = std::str::from_utf8(&payload_bytes) {
                    if let Ok(v) = serde_json::from_str::<serde_json::Value>(payload_str) {
                        actual_email = v.get("email").and_then(|x| x.as_str()).unwrap_or("").to_string();
                        actual_sub = v.get("sub").and_then(|x| x.as_str()).unwrap_or("").to_string();
                        given_name = v.get("given_name").and_then(|x| x.as_str()).map(|s| s.to_string());
                        family_name = v.get("family_name").and_then(|x| x.as_str()).map(|s| s.to_string());
                        display_name = v.get("name").and_then(|x| x.as_str()).map(|s| s.to_string());
                        picture = v.get("picture").and_then(|x| x.as_str()).map(|s| s.to_string());
                        console_log!("🔐 CALLBACK: Extracted user from ID token: {}", actual_email);
                    }
                }
            }
        }
    }

    if actual_email.is_empty() || actual_sub.is_empty() {
        console_log!("🔐 CALLBACK: ID token missing/incomplete; fetching userinfo via access_token");
        let mut init2 = RequestInit::new();
        init2.method = Method::Get;
        let headers2 = Headers::new();
        headers2.set("Authorization", &format!("Bearer {}", token_response.access_token))
            .map_err(|_| AppError::ExternalServiceError("Failed to set Authorization header".into()))?;
        headers2.set("Accept", "application/json")
            .map_err(|_| AppError::ExternalServiceError("Failed to set Accept header".into()))?;
        init2.headers = headers2;
        let request2 = Request::new_with_init("https://www.googleapis.com/oauth2/v2/userinfo", &init2)
            .map_err(|e| AppError::ExternalServiceError(format!("Failed to create userinfo request: {:?}", e)))?;
        let mut resp2 = Fetch::Request(request2).send().await
            .map_err(|e| AppError::ExternalServiceError(format!("Userinfo request failed: {:?}", e)))?;
        if resp2.status_code() != 200 {
            let txt = resp2.text().await.unwrap_or_default();
            error!("Userinfo failed ({}): {}", resp2.status_code(), txt);
            let redirect_url = format!("{}/auth/callback?error={}", state.config.base_url.trim_end_matches('/'), urlencoding::encode("Failed to get user info"));
            return Ok(Redirect::temporary(&redirect_url).into_response());
        }
        let ui: crate::dto::oauth::GoogleUserInfo = resp2.json().await
            .map_err(|e| AppError::ExternalServiceError(format!("Failed to parse userinfo: {:?}", e)))?;
        actual_email = ui.email.clone();
        actual_sub = ui.sub.clone();
        given_name = ui.given_name.clone();
        family_name = ui.family_name.clone();
        display_name = ui.name.clone();
        picture = ui.picture.clone();
        console_log!("🔐 CALLBACK: Retrieved userinfo for {}", actual_email);
    }

    // Persist user and account linkage
    let user_id = create_or_find_user_complete(
        &state.database,
        &actual_email,
        given_name.as_deref(),
        family_name.as_deref(),
        display_name.as_deref(),
        picture.as_deref(),
    ).await?;
    console_log!("🔐 CALLBACK: User created/found: {}", user_id);

    let linked_account_id = create_or_find_linked_account_complete(
        &state.database,
        user_id,
        &actual_sub,
        &crate::dto::oauth::GoogleUserInfo {
            sub: actual_sub.clone(),
            email: actual_email.clone(),
            email_verified: true,
            name: display_name.clone(),
            given_name: given_name.clone(),
            family_name: family_name.clone(),
            picture: picture.clone(),
            locale: None,
        },
    ).await?;
    console_log!("🔐 CALLBACK: Linked account created/found: {}", linked_account_id);

    let session_token_id = store_oauth_tokens_complete(
        &state.database,
        linked_account_id,
        &token_response.access_token,
        token_response.refresh_token.as_deref(),
        token_response.expires_in,
    ).await?;
    console_log!("🔐 CALLBACK: Session token stored: {}", session_token_id);

    // Create session row
    let expires_at = chrono::Utc::now() + chrono::Duration::hours(24);
    let session_row_id = Uuid::new_v4();
    let session_query = "
        INSERT INTO sessions_table (session_id, user_id, token_id, expires_at, created_at)
        VALUES ($1, $2, $3, $4, NOW())
    ";
    state.database.execute(session_query, &[&session_row_id, &user_id, &session_token_id, &expires_at]).await
        .map_err(|e| AppError::DatabaseError(format!("Failed to create session: {}", e)))?;
    console_log!("🔐 CALLBACK: Session stored in DB: {}", session_row_id);

    // Set session cookie and redirect to app
    use axum::http::HeaderMap;
    let session_cookie_value = format!("{}:{}", user_id, session_token_id);
    let session = crate::middleware::session::SimpleSessionResponse::new(user_id.to_string(), session_token_id.to_string());
    let secure = state.config.environment.to_lowercase() != "development";
    let mut headers: HeaderMap = session.create_headers(secure);

    let redirect_after = oauth_state.redirect_after_login.unwrap_or("/dashboard".to_string());
    let redirect_url = format!("{}{}", state.config.base_url.trim_end_matches('/'), redirect_after);
    console_log!("🔐 CALLBACK: Redirecting to {} with Set-Cookie (session_id={})", redirect_url, session_cookie_value);

    // Build a response merging headers with Redirect
    let redirect = Redirect::temporary(&redirect_url);
    let mut response = redirect.into_response();
    // Merge Set-Cookie header(s)
    for (k, v) in headers.iter() {
        response.headers_mut().append(k, v.clone());
    }
    Ok(response)
}
*/

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

/// Get current user from session - GET /api/auth/user
/// This endpoint validates the session and returns user data in the format expected by the frontend
#[axum::debug_handler]
pub async fn get_current_user_improved(
    State(state): State<AppState>,
    req: axum::http::Request<axum::body::Body>,
) -> Result<impl IntoResponse, AppError> {
    
    let (parts, _body) = req.into_parts();
    
    // Extract session token from cookies
    let session_token = extract_session_token_from_cookies(&parts)
        .ok_or_else(|| AppError::AuthenticationError("No session found".to_string()))?;
    
    console_log!("🔐 IMPROVED: Validating session token for /api/auth/user");
    
    // Query database to validate session token and get user info
    match validate_session_and_get_user(&state.database, &session_token).await {
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

/// Extract session token from HTTP cookies
fn extract_session_token_from_cookies(parts: &axum::http::request::Parts) -> Option<String> {
    
    parts
        .headers
        .get(axum::http::header::COOKIE)?
        .to_str()
        .ok()?
        .split(';')
        .find_map(|cookie| {
            let cookie = cookie.trim();
            if cookie.starts_with("session_id=") {
                cookie.split('=').nth(1).map(|s| s.trim().to_string())
            } else {
                None
            }
        })
}

/// Validate session token and return user data
async fn validate_session_and_get_user(
    database: &crate::database::Database,
    session_token: &str,
) -> Result<User, String> {
    
    // Parse cookie format: user_id:token_id
    let parts: Vec<&str> = session_token.split(':').collect();
    if parts.len() != 2 {
        return Err("Invalid session cookie format".to_string());
    }
    
    let user_id_str = parts[0];
    let token_id_str = parts[1];
    
    // Parse user_id as UUID
    let user_id = Uuid::parse_str(user_id_str)
        .map_err(|e| format!("Invalid user ID format: {}", e))?;
    let token_id = Uuid::parse_str(token_id_str)
        .map_err(|e| format!("Invalid token ID format: {}", e))?;
    
    console_log!("🔐 IMPROVED: Validating session for user_id: {}, token_id: {}", user_id, token_id);
    
    const SESSION_TOKEN_TYPE: i16 = 1; // From schema: session token type
    
    // Query to get user info by validating both user_id and token_id
    // The token should belong to the user and still be valid
    let query = "
        SELECT u.id, u.primary_email, u.display_name, u.avatar_url
        FROM users u
        JOIN tokens t ON t.user_id = u.id
        WHERE u.id = $1 
        AND t.id = $2
        AND t.type_id = $3 
        AND t.expires_at > NOW()
    ";
    
    match database.query_one(query, &[&user_id, &token_id, &SESSION_TOKEN_TYPE]).await {
        Ok(row) => {
            let user_id: Uuid = row.get(0);
            let email: String = row.get(1);
            let display_name: Option<String> = row.get(2);
            let avatar_url: Option<String> = row.get(3);
            
            console_log!("🔐 IMPROVED: Session validation successful for user: {} ({})", email, user_id);
            
            Ok(User {
                id: user_id.to_string(),
                email,
                name: display_name,
                picture: avatar_url,
            })
        }
        Err(e) => {
            console_log!("🔐 IMPROVED: Session validation failed for user_id: {}, token_id: {} - {}", user_id, token_id, e);
            Err(format!("Session validation failed: {}", e))
        }
    }
}

/// Create or find user in `users` table - complete version for PKCE callback
async fn create_or_find_user_complete(
    database: &crate::database::Database,
    email: &str,
    first_name: Option<&str>,
    last_name: Option<&str>,
    display_name: Option<&str>,
    avatar_url: Option<&str>,
) -> Result<Uuid, AppError> {
    // First try to find existing user by primary_email
    let find_query = "SELECT id FROM users WHERE primary_email = $1";
    
    match database.query_one(find_query, &[&email]).await {
        Ok(row) => {
            let user_id: Uuid = row.get(0);
            
            // Update last_login_at and user info if needed
            let update_query = "
                UPDATE users 
                SET last_login_at = NOW(), 
                    display_name = COALESCE($2, display_name),
                    avatar_url = COALESCE($3, avatar_url),
                    updated_at = NOW()
                WHERE id = $1
            ";
            
            let _ = database.execute(update_query, &[&user_id, &display_name, &avatar_url]).await;
            
            console_log!("🔐 IMPROVED: Found existing user: {}", user_id);
            Ok(user_id)
        }
        Err(_) => {
            // Create new user with proper schema columns
            let user_id = Uuid::new_v4();
            let create_query = "
                INSERT INTO users (id, primary_email, first_name, last_name, display_name, avatar_url, is_verified, created_at, last_login_at)
                VALUES ($1, $2, $3, $4, $5, $6, true, NOW(), NOW())
                RETURNING id
            ";
            
            let row = database.query_one(create_query, &[
                &user_id, 
                &email, 
                &first_name, 
                &last_name, 
                &display_name, 
                &avatar_url
            ]).await
                .map_err(|e| AppError::DatabaseError(format!("Failed to create user: {}", e)))?;
            
            let created_user_id: Uuid = row.get(0);
            console_log!("🔐 IMPROVED: Created new user: {}", created_user_id);
            Ok(created_user_id)
        }
    }
}

/// Create or find linked_account for Google provider - complete version for PKCE callback
async fn create_or_find_linked_account_complete(
    database: &crate::database::Database,
    user_id: Uuid,
    provider_user_id: &str,
    user_info: &crate::dto::oauth::GoogleUserInfo,
) -> Result<Uuid, AppError> {
    const GOOGLE_PROVIDER_ID: i16 = 2; // From schema: Google OAuth provider
    
    // First try to find existing linked_account
    let find_query = "SELECT id FROM linked_accounts WHERE user_id = $1 AND provider_id = $2 AND provider_user_id = $3";
    
    match database.query_one(find_query, &[&user_id, &GOOGLE_PROVIDER_ID, &provider_user_id]).await {
        Ok(row) => {
            let linked_account_id: Uuid = row.get(0);
            
            // Update provider profile data and last_login_at
            let provider_profile_data = serde_json::json!({
                "sub": user_info.sub,
                "email_verified": user_info.email_verified,
                "picture": user_info.picture,
                "locale": user_info.locale,
                "given_name": user_info.given_name,
                "family_name": user_info.family_name
            });
            
            let update_query = "
                UPDATE linked_accounts 
                SET last_login_at = NOW(), 
                    provider_email = $2,
                    provider_display_name = $3,
                    provider_avatar_url = $4,
                    provider_profile_data = $5,
                    updated_at = NOW()
                WHERE id = $1
            ";
            
            database.execute(update_query, &[
                &linked_account_id,
                &user_info.email,
                &user_info.name,
                &user_info.picture,
                &provider_profile_data
            ]).await
                .map_err(|e| AppError::DatabaseError(format!("Failed to update linked account: {}", e)))?;
            
            console_log!("🔐 IMPROVED: Found existing linked account: {}", linked_account_id);
            Ok(linked_account_id)
        }
        Err(_) => {
            // Create new linked_account
            let linked_account_id = Uuid::new_v4();
            let provider_profile_data = serde_json::json!({
                "sub": user_info.sub,
                "email_verified": user_info.email_verified,
                "picture": user_info.picture,
                "locale": user_info.locale,
                "given_name": user_info.given_name,
                "family_name": user_info.family_name
            });
            
            let create_query = "
                INSERT INTO linked_accounts (
                    id, user_id, provider_id, provider_user_id, 
                    provider_email, provider_display_name, provider_avatar_url, provider_profile_data,
                    is_active, connected_at, last_login_at
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, true, NOW(), NOW())
                RETURNING id
            ";
            
            let row = database.query_one(create_query, &[
                &linked_account_id, 
                &user_id, 
                &GOOGLE_PROVIDER_ID, 
                &provider_user_id,
                &user_info.email,
                &user_info.name,
                &user_info.picture,
                &provider_profile_data
            ]).await
                .map_err(|e| AppError::DatabaseError(format!("Failed to create linked account: {}", e)))?;
            
            let created_linked_account_id: Uuid = row.get(0);
            console_log!("🔐 IMPROVED: Created new linked account: {}", created_linked_account_id);
            Ok(created_linked_account_id)
        }
    }
}

/// Store OAuth tokens and create session token - complete version for PKCE callback
async fn store_oauth_tokens_complete(
    database: &crate::database::Database,
    linked_account_id: Uuid,
    access_token: &str,
    refresh_token: Option<&str>,
    expires_in: i64,
) -> Result<Uuid, AppError> {
    const OAUTH_ACCESS_TOKEN_TYPE: i16 = 2; // From schema: oauth_access token type
    const OAUTH_REFRESH_TOKEN_TYPE: i16 = 3; // From schema: oauth_refresh token type
    const SESSION_TOKEN_TYPE: i16 = 1; // From schema: session token type
    
    // Calculate expiration time
    let expires_at = chrono::Utc::now() + chrono::Duration::seconds(expires_in);
    
    // Store access token (linked to linked_account, not user)
    let access_token_id = Uuid::new_v4();
    let store_access_query = "
        INSERT INTO tokens (id, linked_account_id, type_id, value, expires_at, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
        ON CONFLICT (value) DO UPDATE SET
            expires_at = EXCLUDED.expires_at,
            updated_at = NOW()
    ";
    
    database.execute(store_access_query, &[
        &access_token_id,
        &linked_account_id,
        &OAUTH_ACCESS_TOKEN_TYPE,
        &access_token,
        &expires_at
    ]).await.map_err(|e| AppError::DatabaseError(format!("Failed to store access token: {}", e)))?;
    
    console_log!("🔐 IMPROVED: Stored access token");

    // Store refresh token if provided
    if let Some(refresh_token) = refresh_token {
        let refresh_token_id = Uuid::new_v4();
        let refresh_expires_at = chrono::Utc::now() + chrono::Duration::days(30); // 30 days for refresh token
        
        let store_refresh_query = "
            INSERT INTO tokens (id, linked_account_id, type_id, value, expires_at, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
            ON CONFLICT (value) DO UPDATE SET
                expires_at = EXCLUDED.expires_at,
                updated_at = NOW()
        ";
        
        database.execute(store_refresh_query, &[
            &refresh_token_id,
            &linked_account_id,
            &OAUTH_REFRESH_TOKEN_TYPE,
            &refresh_token,
            &refresh_expires_at
        ]).await.map_err(|e| AppError::DatabaseError(format!("Failed to store refresh token: {}", e)))?;
        
        console_log!("🔐 IMPROVED: Stored refresh token");
    }

    // Create a session token for this user session
    let session_token_id = Uuid::new_v4();
    let session_token_value = format!("session_{}", Uuid::new_v4());
    let session_expires_at = chrono::Utc::now() + chrono::Duration::hours(24);
    
    // Get user_id from linked_account for session token (user-scoped token)
    let user_id_query = "SELECT user_id FROM linked_accounts WHERE id = $1";
    let user_row = database.query_one(user_id_query, &[&linked_account_id]).await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get user_id: {}", e)))?;
    let user_id: Uuid = user_row.get(0);
    
    let store_session_token_query = "
        INSERT INTO tokens (id, user_id, type_id, value, expires_at, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
    ";
    
    database.execute(store_session_token_query, &[
        &session_token_id,
        &user_id,
        &SESSION_TOKEN_TYPE,
        &session_token_value,
        &session_expires_at
    ]).await.map_err(|e| AppError::DatabaseError(format!("Failed to store session token: {}", e)))?;
    
    console_log!("🔐 IMPROVED: Created session token");

    Ok(session_token_id)
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

