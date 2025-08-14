//! Authentication service - Business logic layer for authentication
//! Acts as intermediary between handlers and repositories

use std::sync::Arc;
use uuid::Uuid;
use chrono::{Utc, Duration};
use serde_json;
use crate::{
    utils::error::AppError,
    database::Database,
    repository::auth::AuthRepository,
    handler::auth::{User, UserInfo, FrontendOAuthSubmission, OAuthResponse},
};
use crate::service::gmail::sanitize_log_snippet;
use worker::console_log;

/// Authentication service for handling OAuth and session management
pub struct AuthService {
    auth_repository: AuthRepository,
    database: Arc<Database>,
}

impl AuthService {
    /// Create new auth service with database connection
    pub fn new(database: Arc<Database>) -> Self {
        Self {
            auth_repository: AuthRepository::new(Arc::clone(&database)),
            database,
        }
    }

    /// Validate session and get current user
    pub async fn get_current_user(&self, session_token: &str) -> Result<User, AppError> {
        self.auth_repository.validate_session_and_get_user(session_token).await
    }

    /// Handle logout - invalidate session and tokens
    pub async fn logout(&self, session_token: Option<String>) -> Result<(), AppError> {
        if let Some(token) = session_token {
            self.auth_repository.logout(&token).await?;
        }
        Ok(())
    }

    /// Handle OAuth login from frontend
    pub async fn handle_frontend_oauth(&self, data: FrontendOAuthSubmission) -> Result<OAuthResponse, AppError> {
        console_log!("🔐 FRONTEND-OAUTH: ====== Starting OAuth Flow ======");
        console_log!("🔐 FRONTEND-OAUTH: User email: {}", data.user_info.email);
        console_log!("🔐 FRONTEND-OAUTH: User ID (sub): {}", data.user_info.id);
    if let Some(scope_str) = &data.scope { console_log!("🔐 FRONTEND-OAUTH: Raw scopes string: {}", scope_str); } else { console_log!("🔐 FRONTEND-OAUTH: No scopes provided by frontend submission"); }
        
        // Step 1: Create or find user
        let user_id = self.auth_repository.create_or_find_user(
            &data.user_info.email,
            data.user_info.given_name.as_deref(),
            data.user_info.family_name.as_deref(),
            data.user_info.name.as_deref(),
            data.user_info.picture.as_deref(),
        ).await?;

        console_log!("🔐 FRONTEND-OAUTH: ✅ User created/found: {}", user_id);

        // Step 2: Create or update linked account for Google
        const GOOGLE_PROVIDER_ID: i16 = 2; // From schema
        let provider_profile_data = serde_json::json!({
            "id": data.user_info.id,
            "email_verified": data.user_info.email_verified,
            "picture": data.user_info.picture,
            "locale": data.user_info.locale,
            "given_name": data.user_info.given_name,
            "family_name": data.user_info.family_name
        });

        let linked_account_id = self.auth_repository.create_or_find_linked_account(
            user_id,
            GOOGLE_PROVIDER_ID,
            &data.user_info.id,
            &data.user_info.email,
            data.user_info.name.as_deref(),
            data.user_info.picture.as_deref(),
            provider_profile_data,
        ).await?;

        console_log!("🔐 FRONTEND-OAUTH: ✅ Linked account: {}", linked_account_id);

        // Step 3: Enforce single active session per user
        self.auth_repository.enforce_single_session(user_id).await?;
        console_log!("🔐 FRONTEND-OAUTH: ✅ Single session policy enforced");

        // Step 4: Store OAuth tokens and create session token
        let session_token_id = self.auth_repository.store_oauth_tokens(
            linked_account_id,
            user_id,
            &data.access_token,
            data.refresh_token.as_deref(),
            data.expires_in,
        ).await?;

        console_log!("🔐 FRONTEND-OAUTH: ✅ Session token created: {}", session_token_id);

        // Step 4b: Persist scopes for access token if provided
    if let Some(scope_str) = &data.scope {
            let scopes: Vec<String> = scope_str
                .split(|c: char| c.is_whitespace())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect();
            console_log!("🔐 FRONTEND-OAUTH: Parsed {} scopes", scopes.len());
            if !scopes.is_empty() {
                match self.auth_repository.persist_access_token_scopes(linked_account_id, &scopes).await {
                    Ok(count) => {
                        let has_gmail = scopes.iter().any(|s| s == "https://www.googleapis.com/auth/gmail.readonly");
                        console_log!("🔐 FRONTEND-OAUTH: ✅ Persisted {} scopes (gmail.readonly present? {} )", count, has_gmail);
                        if !has_gmail { console_log!("⚠️ FRONTEND-OAUTH: gmail.readonly scope missing from persisted set"); }
                    }
                    Err(e) => console_log!("⚠️ FRONTEND-OAUTH: Failed to persist scopes: {}", e),
                }
            } else {
        // Attempt tokeninfo fallback (fire & forget)
        self.spawn_scope_persistence_fallback(data.access_token.clone(), linked_account_id);
            }
        } else {
        // No scopes in payload – attempt tokeninfo fallback (fire & forget)
        self.spawn_scope_persistence_fallback(data.access_token.clone(), linked_account_id);
        }

        // Step 5: Create session in sessions_table
        let session_id = Uuid::new_v4();
        let expires_at = Utc::now() + Duration::hours(24);
        
        self.auth_repository.create_session(
            session_id,
            user_id,
            session_token_id,
            expires_at,
        ).await?;

        console_log!("🔐 FRONTEND-OAUTH: ✅ Session created in database");
        console_log!("🔐 FRONTEND-OAUTH: Session format for cookie: {}:{}", user_id, session_token_id);

        // Step 6: Persist simplified session snapshot (account_sessions) for Gmail 2FA auto-retrieval
        // This is separate from sessions_table; it stores JSON with user_email & tokens looked up by SessionRepository
        {
            use crate::repository::session::{SessionRepository, UserSessionData};
            let session_repo = SessionRepository::new(Arc::clone(&self.database));
            let session_cookie_id = format!("{}:{}", user_id, session_token_id);
            let token_expires_at = Utc::now() + Duration::seconds(data.expires_in);
            let user_session_data = UserSessionData {
                user_id: user_id.to_string(),
                user_email: data.user_info.email.clone(),
                user_name: data.user_info.name.clone().unwrap_or_else(|| data.user_info.email.clone()),
                oauth_provider: "google".to_string(),
                access_token: Some(data.access_token.clone()),
                refresh_token: data.refresh_token.clone(),
                token_expires_at: Some(token_expires_at),
                user_agent: None,
                ip_address: None,
                axiom_access_token: None,
                axiom_refresh_token: None,
                axiom_user_id: None,
                axiom_connected_at: None,
            };
            // TTL 24h (match cookie) - if existing, upsert
            if let Err(e) = session_repo.create_user_session(session_cookie_id.clone(), user_session_data, 24 * 60 * 60).await {
                console_log!("⚠️ FRONTEND-OAUTH: Failed to persist account_session for Gmail OTP: {}", e);
            } else {
                console_log!("🔐 FRONTEND-OAUTH: ✅ Stored account_session JSON with Google tokens for Gmail OTP");
            }
        }

        // Return response
        let response = OAuthResponse {
            success: true,
            session_id: format!("{}:{}", user_id, session_token_id),
            user_email: data.user_info.email.clone(),
            expires_at: expires_at.to_rfc3339(),
        };

        console_log!("🔐 FRONTEND-OAUTH: ====== OAuth Flow Complete ======");
        Ok(response)
    }

    /// Create or find user (simplified version for backward compatibility)
    pub async fn create_or_find_user(
        &self,
        email: &str,
        first_name: Option<&str>,
        last_name: Option<&str>,
        display_name: Option<&str>,
        avatar_url: Option<&str>,
    ) -> Result<Uuid, AppError> {
        self.auth_repository.create_or_find_user(
            email,
            first_name,
            last_name,
            display_name,
            avatar_url,
        ).await
    }

    /// Create or find linked account (simplified version for backward compatibility)
    pub async fn create_or_find_linked_account(
        &self,
        user_id: Uuid,
        provider_user_id: &str,
        user_info: &UserInfo,
    ) -> Result<Uuid, AppError> {
        const GOOGLE_PROVIDER_ID: i16 = 2;
        
        let provider_profile_data = serde_json::json!({
            "id": user_info.id,
            "email_verified": user_info.email_verified,
            "picture": user_info.picture,
            "locale": user_info.locale,
            "given_name": user_info.given_name,
            "family_name": user_info.family_name
        });

        self.auth_repository.create_or_find_linked_account(
            user_id,
            GOOGLE_PROVIDER_ID,
            provider_user_id,
            &user_info.email,
            user_info.name.as_deref(),
            user_info.picture.as_deref(),
            provider_profile_data,
        ).await
    }

    /// Store OAuth tokens (simplified version for backward compatibility)
    pub async fn store_oauth_tokens(
        &self,
        linked_account_id: Uuid,
        user_id: Uuid,
        access_token: &str,
        refresh_token: Option<&str>,
        expires_in: i64,
    ) -> Result<Uuid, AppError> {
        self.auth_repository.store_oauth_tokens(
            linked_account_id,
            user_id,
            access_token,
            refresh_token,
            expires_in,
        ).await
    }
    // Fire-and-forget non-Send tokeninfo scope fallback (runs on WASM local task without blocking Send future)
    fn spawn_scope_persistence_fallback(&self, access_token: String, linked_account_id: uuid::Uuid) {
        let db = Arc::clone(&self.database);
        wasm_bindgen_futures::spawn_local(async move {
            console_log!("🔐 FRONTEND-OAUTH: (spawn) Attempting tokeninfo scope fallback");
            let url = format!("https://www.googleapis.com/oauth2/v3/tokeninfo?access_token={}", urlencoding::encode(&access_token));
            let mut init = worker::RequestInit::new();
            init.with_method(worker::Method::Get);
            let request = match worker::Request::new_with_init(&url, &init) { Ok(r) => r, Err(e) => { console_log!("⚠️ FRONTEND-OAUTH: (spawn) tokeninfo request build failed: {}", e); return; } };
            let mut resp = match worker::Fetch::Request(request).send().await { Ok(r) => r, Err(e) => { console_log!("⚠️ FRONTEND-OAUTH: (spawn) tokeninfo fetch failed: {}", e); return; } };
            if resp.status_code() != 200 {
                let body = resp.text().await.unwrap_or_default();
                console_log!("⚠️ FRONTEND-OAUTH: (spawn) tokeninfo non-200 status={} body={}...", resp.status_code(), sanitize_log_snippet(&body));
                return;
            }
            let text = match resp.text().await { Ok(t) => t, Err(e) => { console_log!("⚠️ FRONTEND-OAUTH: (spawn) tokeninfo read failed: {}", e); return; } };
            let json: serde_json::Value = match serde_json::from_str(&text) { Ok(j) => j, Err(e) => { console_log!("⚠️ FRONTEND-OAUTH: (spawn) tokeninfo parse failed: {}", e); return; } };
            if let Some(scope_str) = json.get("scope").and_then(|v| v.as_str()) {
                let scopes: Vec<String> = scope_str.split_whitespace().map(|s| s.to_string()).collect();
                if scopes.is_empty() { console_log!("🔐 FRONTEND-OAUTH: (spawn) tokeninfo returned empty scope list"); return; }
                console_log!("🔐 FRONTEND-OAUTH: (spawn) tokeninfo scopes fetched: {}", scopes.join(","));
                let repo = crate::repository::auth::AuthRepository::new(db);
                match repo.persist_access_token_scopes(linked_account_id, &scopes).await {
                    Ok(count) => {
                        let has_gmail = scopes.iter().any(|s| s == "https://www.googleapis.com/auth/gmail.readonly");
                        console_log!("🔐 FRONTEND-OAUTH: (spawn) ✅ Fallback persisted {} scopes (gmail.readonly present? {} )", count, has_gmail);
                        if !has_gmail { console_log!("⚠️ FRONTEND-OAUTH: (spawn) gmail.readonly not found in tokeninfo scopes"); }
                    }
                    Err(e) => console_log!("⚠️ FRONTEND-OAUTH: (spawn) Failed to persist fallback scopes: {}", e),
                }
            } else {
                console_log!("🔐 FRONTEND-OAUTH: (spawn) tokeninfo response missing 'scope' field");
            }
        });
    }
}