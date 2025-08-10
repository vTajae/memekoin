//! OAuth handler that receives tokens from frontend
//! Frontend handles Google API calls to avoid Send trait issues in WASM

use axum::{
    extract::{State, Json},
    http::{StatusCode, HeaderMap},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use worker::console_log;

use crate::{
    error::AppError,
    state::AppState,
};

/// OAuth submission from frontend (includes tokens and user info)
#[derive(Deserialize)]
pub struct FrontendOAuthSubmission {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: i64,
    pub state: String,
    pub code: Option<String>,
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

/// Handle OAuth tokens from frontend (frontend already exchanged with Google)
pub async fn handle_frontend_oauth(
    State(state): State<AppState>,
    Json(data): Json<FrontendOAuthSubmission>,
) -> Result<impl IntoResponse, AppError> {
    console_log!("🔐 FRONTEND-OAUTH: ====== Starting OAuth Flow ======");
    console_log!("🔐 FRONTEND-OAUTH: Received tokens from frontend");
    console_log!("🔐 FRONTEND-OAUTH: User email: {}", data.user_info.email);
    console_log!("🔐 FRONTEND-OAUTH: User ID (sub): {}", data.user_info.id);
    console_log!("🔐 FRONTEND-OAUTH: User name: {:?}", data.user_info.name);
    console_log!("🔐 FRONTEND-OAUTH: Access token present: {}", !data.access_token.is_empty());
    console_log!("🔐 FRONTEND-OAUTH: Refresh token present: {}", data.refresh_token.is_some());
    console_log!("🔐 FRONTEND-OAUTH: Token expires in: {} seconds", data.expires_in);

    // Step 1: Validate state (if needed - for CSRF protection)
    // For now, we'll trust the frontend since it handled the OAuth flow

    // Step 2: Create or find user
    let user_id = create_or_find_user(
        &state.database,
        &data.user_info.email,
        data.user_info.given_name.as_deref(),
        data.user_info.family_name.as_deref(),
        data.user_info.name.as_deref(),
        data.user_info.picture.as_deref(),
    ).await?;

    console_log!("🔐 FRONTEND-OAUTH: ✅ User created/found: {}", user_id);
    console_log!("🔐 FRONTEND-OAUTH: User email in DB: {}", data.user_info.email);

    // Step 3: Create or update linked account
    let linked_account_id = create_or_find_linked_account(
        &state.database,
        user_id,
        &data.user_info.id,
        &data.user_info,
    ).await?;

    console_log!("🔐 FRONTEND-OAUTH: ✅ Linked account: {}", linked_account_id);
    console_log!("🔐 FRONTEND-OAUTH: Provider: Google (ID=2)");
    console_log!("🔐 FRONTEND-OAUTH: Provider user ID: {}", data.user_info.id);

    // Step 4: Enforce single active session per user BEFORE creating a new session token
    // 4a. Delete any existing sessions for this user
    let delete_sessions_query = "DELETE FROM sessions_table WHERE user_id = $1";
    let _ = state.database.execute(delete_sessions_query, &[&user_id]).await;

    // 4b. Delete any existing session tokens for this user (type_id = 1)
    let delete_tokens_query = "DELETE FROM tokens WHERE user_id = $1 AND type_id = 1";
    let _ = state.database.execute(delete_tokens_query, &[&user_id]).await;

    // Step 4: Store OAuth tokens
    let session_token_id = store_oauth_tokens(
        &state.database,
        linked_account_id,
        &data.access_token,
        data.refresh_token.as_deref(),
        data.expires_in,
    ).await?;

    console_log!("🔐 FRONTEND-OAUTH: ✅ Session token created: {}", session_token_id);
    console_log!("🔐 FRONTEND-OAUTH: Tokens stored - Access: ✓, Refresh: {}, Session: ✓",
        if data.refresh_token.is_some() { "✓" } else { "✗" });


    // 5c. Create a new session
    let session_id = Uuid::new_v4();
    let expires_at = chrono::Utc::now() + chrono::Duration::hours(24);

    let session_query = "
        INSERT INTO sessions_table (session_id, user_id, token_id, expires_at, created_at)
        VALUES ($1, $2, $3, $4, NOW())
    ";

    state.database.execute(session_query, &[
        &session_id,
        &user_id,
        &session_token_id,
        &expires_at
    ]).await.map_err(|e| AppError::DatabaseError(format!("Failed to create session: {}", e)))?;

    console_log!("🔐 FRONTEND-OAUTH: ✅ Session created in database (single-session policy enforced)");
    console_log!("🔐 FRONTEND-OAUTH: Session ID: {}", session_id);
    console_log!("🔐 FRONTEND-OAUTH: Session expires at: {}", expires_at.to_rfc3339());
    console_log!("🔐 FRONTEND-OAUTH: Session format for cookie: {}:{}", user_id, session_token_id);

    // Return response
    let response = OAuthResponse {
        success: true,
        session_id: format!("{}:{}", user_id, session_token_id),
        user_email: data.user_info.email.clone(),
        expires_at: expires_at.to_rfc3339(),
    };

    console_log!("🔐 FRONTEND-OAUTH: ====== OAuth Flow Complete ======");
    console_log!("🔐 FRONTEND-OAUTH: Response - Success: {}, Session: {}",
        response.success, response.session_id);

    // Build response with Set-Cookie header to persist session in browser
    let session_cookie = crate::middleware::session::SimpleSessionResponse::new(
        user_id.to_string(),
        session_token_id.to_string(),
    );
    let headers: HeaderMap = session_cookie.create_headers(state.config.environment.to_lowercase() != "development");

    // Keep JSON body for client-side confirmation, attach Set-Cookie via headers
    let mut resp = (StatusCode::OK, Json(response)).into_response();
    for (k, v) in headers.iter() {
        resp.headers_mut().append(k.clone(), v.clone());
    }

    Ok(resp)
}

// Helper functions (similar to oauth_improved.rs but without Google API calls)

async fn create_or_find_user(
    database: &crate::database::Database,
    email: &str,
    first_name: Option<&str>,
    last_name: Option<&str>,
    display_name: Option<&str>,
    avatar_url: Option<&str>,
) -> Result<Uuid, AppError> {
    console_log!("📄 DB: Looking for existing user with email: {}", email);
    let find_query = "SELECT id FROM users WHERE primary_email = $1";

    match database.query_one(find_query, &[&email]).await {
        Ok(row) => {
            let user_id: Uuid = row.get(0);

            let update_query = "
                UPDATE users
                SET last_login_at = NOW(),
                    display_name = COALESCE($2, display_name),
                    avatar_url = COALESCE($3, avatar_url),
                    updated_at = NOW()
                WHERE id = $1
            ";

            let _ = database.execute(update_query, &[&user_id, &display_name, &avatar_url]).await;

            console_log!("🔐 FRONTEND-OAUTH: Found existing user: {}", user_id);
            console_log!("📄 DB: Updated user's last_login_at and display info");
            Ok(user_id)
        }
        Err(_) => {
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
            console_log!("🔐 FRONTEND-OAUTH: Created new user: {}", created_user_id);
            console_log!("📄 DB: Inserted new user - email: {}, first: {:?}, last: {:?}",
                email, first_name, last_name);
            Ok(created_user_id)
        }
    }
}

async fn create_or_find_linked_account(
    database: &crate::database::Database,
    user_id: Uuid,
    provider_user_id: &str,
    user_info: &UserInfo,
) -> Result<Uuid, AppError> {
    const GOOGLE_PROVIDER_ID: i16 = 2;

    console_log!("📄 DB: Looking for linked account - user: {}, provider: Google, provider_user: {}",
        user_id, provider_user_id);
    let find_query = "SELECT id FROM linked_accounts WHERE user_id = $1 AND provider_id = $2 AND provider_user_id = $3";

    match database.query_one(find_query, &[&user_id, &GOOGLE_PROVIDER_ID, &provider_user_id]).await {
        Ok(row) => {
            let linked_account_id: Uuid = row.get(0);

            let provider_profile_data = serde_json::json!({
                "id": user_info.id,
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

            console_log!("🔐 FRONTEND-OAUTH: Found existing linked account: {}", linked_account_id);
            console_log!("📄 DB: Updated linked account profile data and last_login_at");
            Ok(linked_account_id)
        }
        Err(_) => {
            let linked_account_id = Uuid::new_v4();
            let provider_profile_data = serde_json::json!({
                "id": user_info.id,
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
            console_log!("🔐 FRONTEND-OAUTH: Created new linked account: {}", created_linked_account_id);
            console_log!("📄 DB: Inserted new linked account for Google provider");
            Ok(created_linked_account_id)
        }
    }
}

async fn store_oauth_tokens(
    database: &crate::database::Database,
    linked_account_id: Uuid,
    access_token: &str,
    refresh_token: Option<&str>,
    expires_in: i64,
) -> Result<Uuid, AppError> {
    const OAUTH_ACCESS_TOKEN_TYPE: i16 = 2;
    const OAUTH_REFRESH_TOKEN_TYPE: i16 = 3;
    const SESSION_TOKEN_TYPE: i16 = 1;

    console_log!("📄 DB: Starting token storage for linked_account: {}", linked_account_id);
    let expires_at = chrono::Utc::now() + chrono::Duration::seconds(expires_in);
    console_log!("📄 DB: Token expiration set to: {}", expires_at.to_rfc3339());

    // Store access token
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

    console_log!("📄 DB: ✅ Stored OAuth access token (type=2) for linked_account");

    // Store refresh token if provided
    if let Some(refresh_token) = refresh_token {
        let refresh_token_id = Uuid::new_v4();
        let refresh_expires_at = chrono::Utc::now() + chrono::Duration::days(30);

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

        console_log!("📄 DB: ✅ Stored OAuth refresh token (type=3) for linked_account");
    }

    // Create a session token
    console_log!("📄 DB: Fetching user_id from linked_account");
    let user_id_query = "SELECT user_id FROM linked_accounts WHERE id = $1";
    let user_row = database.query_one(user_id_query, &[&linked_account_id]).await
        .map_err(|e| AppError::DatabaseError(format!("Failed to get user_id: {}", e)))?;
    let user_id: Uuid = user_row.get(0);
    console_log!("📄 DB: Got user_id: {} for session token", user_id);

    let session_token_id = Uuid::new_v4();
    let session_token_value = format!("session_{}", Uuid::new_v4());
    let session_expires_at = chrono::Utc::now() + chrono::Duration::hours(24);

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

    console_log!("🔐 FRONTEND-OAUTH: Created session token");
    console_log!("📄 DB: ✅ Stored session token (type=1) for user: {}", user_id);
    console_log!("📄 DB: Session token ID: {}, Value: {}", session_token_id, session_token_value);

    Ok(session_token_id)
}