//! Authentication repository - Database operations for authentication
//! Handles users, linked accounts, tokens, and sessions

use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
// use serde_json; // (Unused import removed – json! macro invoked with full path where needed)
use crate::{
    utils::error::AppError,
    database::Database,
};
use worker::console_log;

/// Authentication repository for all auth-related database operations
pub struct AuthRepository {
    database: Arc<Database>,
}

impl AuthRepository {
    /// Create new auth repository
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }

    /// Create or find user in users table
    pub async fn create_or_find_user(
        &self,
        email: &str,
        first_name: Option<&str>,
        last_name: Option<&str>,
        display_name: Option<&str>,
        avatar_url: Option<&str>,
    ) -> Result<Uuid, AppError> {
        console_log!("📄 DB: Looking for existing user with email: {}", email);
        let find_query = "SELECT id FROM users WHERE primary_email = $1";
        
        match self.database.query_one(find_query, &[&email]).await {
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
                
                let _ = self.database.execute(update_query, &[&user_id, &display_name, &avatar_url]).await;
                
                console_log!("🔐 Found existing user: {}", user_id);
                console_log!("📄 DB: Updated user's last_login_at and display info");
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
                
                let row = self.database.query_one(create_query, &[
                    &user_id, 
                    &email, 
                    &first_name, 
                    &last_name, 
                    &display_name, 
                    &avatar_url
                ]).await
                    .map_err(|e| AppError::DatabaseError(format!("Failed to create user: {}", e)))?;
                
                let created_user_id: Uuid = row.get(0);
                console_log!("🔐 Created new user: {}", created_user_id);
                console_log!("📄 DB: Inserted new user - email: {}, first: {:?}, last: {:?}",
                    email, first_name, last_name);
                Ok(created_user_id)
            }
        }
    }

    /// Create or find linked_account for OAuth provider
    pub async fn create_or_find_linked_account(
        &self,
        user_id: Uuid,
        provider_id: i16,
        provider_user_id: &str,
        provider_email: &str,
        provider_display_name: Option<&str>,
        provider_avatar_url: Option<&str>,
        provider_profile_data: serde_json::Value,
    ) -> Result<Uuid, AppError> {
        console_log!("📄 DB: Looking for linked account - user: {}, provider: {}, provider_user: {}",
            user_id, provider_id, provider_user_id);
        
        let find_query = "SELECT id FROM linked_accounts WHERE user_id = $1 AND provider_id = $2 AND provider_user_id = $3";
        
        match self.database.query_one(find_query, &[&user_id, &provider_id, &provider_user_id]).await {
            Ok(row) => {
                let linked_account_id: Uuid = row.get(0);
                
                // Update provider profile data and last_login_at
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
                
                self.database.execute(update_query, &[
                    &linked_account_id,
                    &provider_email,
                    &provider_display_name,
                    &provider_avatar_url,
                    &provider_profile_data
                ]).await
                    .map_err(|e| AppError::DatabaseError(format!("Failed to update linked account: {}", e)))?;
                
                console_log!("🔐 Found existing linked account: {}", linked_account_id);
                console_log!("📄 DB: Updated linked account profile data and last_login_at");
                Ok(linked_account_id)
            }
            Err(_) => {
                // Create new linked_account
                let linked_account_id = Uuid::new_v4();
                
                let create_query = "
                    INSERT INTO linked_accounts (
                        id, user_id, provider_id, provider_user_id, 
                        provider_email, provider_display_name, provider_avatar_url, provider_profile_data,
                        is_active, connected_at, last_login_at
                    )
                    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, true, NOW(), NOW())
                    RETURNING id
                ";
                
                let row = self.database.query_one(create_query, &[
                    &linked_account_id, 
                    &user_id, 
                    &provider_id, 
                    &provider_user_id,
                    &provider_email,
                    &provider_display_name,
                    &provider_avatar_url,
                    &provider_profile_data
                ]).await
                    .map_err(|e| AppError::DatabaseError(format!("Failed to create linked account: {}", e)))?;
                
                let created_linked_account_id: Uuid = row.get(0);
                console_log!("🔐 Created new linked account: {}", created_linked_account_id);
                console_log!("📄 DB: Inserted new linked account for provider {}", provider_id);
                Ok(created_linked_account_id)
            }
        }
    }

    /// Store OAuth tokens and create session token
    pub async fn store_oauth_tokens(
        &self,
        linked_account_id: Uuid,
        user_id: Uuid,
        access_token: &str,
        refresh_token: Option<&str>,
        expires_in: i64,
    ) -> Result<Uuid, AppError> {
        const OAUTH_ACCESS_TOKEN_TYPE: i16 = 2;
        const OAUTH_REFRESH_TOKEN_TYPE: i16 = 3;
        const SESSION_TOKEN_TYPE: i16 = 1;
        
        console_log!("📄 DB: Starting token storage for linked_account: {}", linked_account_id);
        let expires_at = Utc::now() + Duration::seconds(expires_in);
        console_log!("📄 DB: Token expiration set to: {}", expires_at.to_rfc3339());
        
        // Store access token (linked to linked_account)
        let access_token_id = Uuid::new_v4();
        // Column-based ON CONFLICT avoids depending on a specific named constraint.
        let store_access_query = "
            INSERT INTO tokens (id, linked_account_id, type_id, value, expires_at, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
            ON CONFLICT (linked_account_id, type_id) DO UPDATE SET
                value = EXCLUDED.value,
                expires_at = EXCLUDED.expires_at,
                updated_at = NOW()
        ";

        self.database.execute(store_access_query, &[&access_token_id, &linked_account_id, &OAUTH_ACCESS_TOKEN_TYPE, &access_token, &expires_at])
            .await.map_err(|e| AppError::DatabaseError(format!("Failed to store access token: {}", e)))?;
        
        console_log!("📄 DB: ✅ Stored OAuth access token (type=2) for linked_account");

        // Store refresh token if provided
        if let Some(refresh_token) = refresh_token {
            let refresh_token_id = Uuid::new_v4();
            let refresh_expires_at = Utc::now() + Duration::days(30); // 30 days for refresh token

            let store_refresh_query = "
                INSERT INTO tokens (id, linked_account_id, type_id, value, expires_at, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
                ON CONFLICT (linked_account_id, type_id) DO UPDATE SET
                    value = EXCLUDED.value,
                    expires_at = EXCLUDED.expires_at,
                    updated_at = NOW()
            ";

            self.database.execute(store_refresh_query, &[&refresh_token_id, &linked_account_id, &OAUTH_REFRESH_TOKEN_TYPE, &refresh_token, &refresh_expires_at])
                .await.map_err(|e| AppError::DatabaseError(format!("Failed to store refresh token: {}", e)))?;
            
            console_log!("📄 DB: ✅ Stored OAuth refresh token (type=3) for linked_account");
        }

        // Create a session token for this user session
        let session_token_id = Uuid::new_v4();
        let session_token_value = format!("session_{}", Uuid::new_v4());
        let session_expires_at = Utc::now() + Duration::hours(24);
        
        let store_session_token_query = "
            INSERT INTO tokens (id, user_id, type_id, value, expires_at, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
        ";
        
        self.database.execute(store_session_token_query, &[
            &session_token_id,
            &user_id,
            &SESSION_TOKEN_TYPE,
            &session_token_value,
            &session_expires_at
        ]).await.map_err(|e| AppError::DatabaseError(format!("Failed to store session token: {}", e)))?;
        
        console_log!("🔐 Created session token");
        console_log!("📄 DB: ✅ Stored session token (type=1) for user: {}", user_id);
        console_log!("📄 DB: Session token ID: {}, Value: {}", session_token_id, session_token_value);

        Ok(session_token_id)
    }

    /// Persist scopes for the current (latest) OAuth access token for a linked account.
    /// This should be called immediately after store_oauth_tokens, before another update occurs.
    pub async fn persist_access_token_scopes(&self, linked_account_id: Uuid, scopes: &[String]) -> Result<usize, AppError> {
        if scopes.is_empty() { return Ok(0); }

        // Retrieve the current access token id (type_id = 2) for this linked account
        let query = "SELECT id FROM tokens WHERE linked_account_id = $1 AND type_id = 2";
        let row = self.database.query_one(query, &[&linked_account_id]).await
            .map_err(|e| AppError::DatabaseError(format!("Failed to lookup access token for scope persistence: {}", e)))?;
        let token_id: Uuid = row.get(0);

        // Insert scopes with ON CONFLICT DO NOTHING to avoid duplicates
        let mut inserted = 0usize;
        for scope in scopes {
            let insert = "INSERT INTO token_scopes (token_id, scope, created_at) VALUES ($1, $2, NOW()) ON CONFLICT DO NOTHING";
            let res = self.database.execute(insert, &[&token_id, scope]).await;
            if res.is_ok() { inserted += 1; }
        }
        console_log!("📄 DB: Persisted {} scopes for access token {}", inserted, token_id);
        Ok(inserted)
    }

    /// Create session in sessions_table
    pub async fn create_session(
        &self,
        session_id: Uuid,
        user_id: Uuid,
        token_id: Uuid,
        expires_at: DateTime<Utc>,
    ) -> Result<(), AppError> {
        let session_query = "
            INSERT INTO sessions_table (session_id, user_id, token_id, expires_at, created_at)
            VALUES ($1, $2, $3, $4, NOW())
        ";

        self.database.execute(session_query, &[
            &session_id,
            &user_id,
            &token_id,
            &expires_at
        ]).await.map_err(|e| AppError::DatabaseError(format!("Failed to create session: {}", e)))?;
        
        console_log!("📄 DB: ✅ Created session in sessions_table");
        Ok(())
    }

    /// Enforce single active session per user
    pub async fn enforce_single_session(&self, user_id: Uuid) -> Result<(), AppError> {
        // Delete any existing sessions for this user
        let delete_sessions_query = "DELETE FROM sessions_table WHERE user_id = $1";
        let _ = self.database.execute(delete_sessions_query, &[&user_id]).await;

        // Delete any existing session tokens for this user (type_id = 1)
        const SESSION_TOKEN_TYPE: i16 = 1;
        let delete_tokens_query = "DELETE FROM tokens WHERE user_id = $1 AND type_id = $2";
        let _ = self.database.execute(delete_tokens_query, &[&user_id, &SESSION_TOKEN_TYPE]).await;
        
        console_log!("📄 DB: Enforced single-session policy for user: {}", user_id);
        Ok(())
    }

    /// Validate session token and return user data
    pub async fn validate_session_and_get_user(
        &self,
        session_token: &str,
    ) -> Result<crate::handler::auth::User, AppError> {
        // Parse cookie format: user_id:token_id
        let parts: Vec<&str> = session_token.split(':').collect();
        if parts.len() != 2 {
            return Err(AppError::AuthenticationError("Invalid session cookie format".to_string()));
        }
        
        let user_id_str = parts[0];
        let token_id_str = parts[1];
        
        // Parse user_id and token_id as UUID
        let user_id = Uuid::parse_str(user_id_str)
            .map_err(|e| AppError::AuthenticationError(format!("Invalid user ID format: {}", e)))?;
        let token_id = Uuid::parse_str(token_id_str)
            .map_err(|e| AppError::AuthenticationError(format!("Invalid token ID format: {}", e)))?;
        
        console_log!("🔐 Validating session for user_id: {}, token_id: {}", user_id, token_id);
        
        const SESSION_TOKEN_TYPE: i16 = 1; // From schema: session token type
        
        // Query to get user info by validating both user_id and token_id
        let query = "
            SELECT u.id, u.primary_email, u.display_name, u.avatar_url
            FROM users u
            JOIN tokens t ON t.user_id = u.id
            WHERE u.id = $1 
            AND t.id = $2
            AND t.type_id = $3 
            AND t.expires_at > NOW()
        ";
        
        match self.database.query_one(query, &[&user_id, &token_id, &SESSION_TOKEN_TYPE]).await {
            Ok(row) => {
                let user_id: Uuid = row.get(0);
                let email: String = row.get(1);
                let display_name: Option<String> = row.get(2);
                let avatar_url: Option<String> = row.get(3);
                
                console_log!("🔐 Session validation successful for user: {} ({})", email, user_id);
                
                Ok(crate::handler::auth::User {
                    id: user_id.to_string(),
                    email,
                    name: display_name,
                    picture: avatar_url,
                })
            }
            Err(e) => {
                console_log!("🔐 Session validation failed for user_id: {}, token_id: {} - {}", user_id, token_id, e);
                Err(AppError::AuthenticationError(format!("Session validation failed: {}", e)))
            }
        }
    }

    /// Logout - invalidate session and tokens
    pub async fn logout(&self, session_token: &str) -> Result<(), AppError> {
        let parts: Vec<&str> = session_token.split(':').collect();
        if parts.len() == 2 {
            let user_id_str = parts[0];
            let token_id_str = parts[1];

            // Parse UUIDs; if parsing fails, still proceed
            let parsed_user = Uuid::parse_str(user_id_str);
            let parsed_token = Uuid::parse_str(token_id_str);

            if let (Ok(_user_id), Ok(token_id)) = (parsed_user, parsed_token) {
                // Invalidate this specific session by token_id
                let _ = self.database
                    .execute(
                        "DELETE FROM sessions_table WHERE token_id = $1",
                        &[&token_id],
                    )
                    .await;

                // Delete the session token (type_id = 1)
                const SESSION_TOKEN_TYPE: i16 = 1;
                let _ = self.database
                    .execute(
                        "DELETE FROM tokens WHERE id = $1 AND type_id = $2",
                        &[&token_id, &SESSION_TOKEN_TYPE],
                    )
                    .await;
                    
                console_log!("📄 DB: Logged out session with token_id: {}", token_id);
            }
        }
        Ok(())
    }

    /// Retrieve latest (non-expired) Google OAuth access token for a user email from tokens table
    pub async fn get_latest_google_access_token(&self, user_email: &str) -> Result<Option<String>, AppError> {
        // provider_id 2 = google, token type 2 = oauth_access
        const GOOGLE_PROVIDER_ID: i16 = 2;
        const OAUTH_ACCESS_TOKEN_TYPE: i16 = 2;
        let query = r#"
            SELECT t.value
            FROM tokens t
            JOIN linked_accounts la ON la.id = t.linked_account_id
            JOIN users u ON u.id = la.user_id
            WHERE u.primary_email ILIKE $1
              AND la.provider_id = $2
              AND t.type_id = $3
              AND (t.expires_at IS NULL OR t.expires_at > NOW())
            ORDER BY t.expires_at DESC NULLS LAST
            LIMIT 1
        "#;
        match self.database.query_one(query, &[&user_email, &GOOGLE_PROVIDER_ID, &OAUTH_ACCESS_TOKEN_TYPE]).await {
            Ok(row) => {
                let value: String = row.get(0);
                Ok(Some(value))
            }
            Err(e) => {
                // If no rows, return None; other errors propagate
                let is_no_rows = e.to_string().contains("no rows");
                if is_no_rows { Ok(None) } else { Err(AppError::DatabaseError(format!("Token lookup failed: {}", e))) }
            }
        }
    }

    /// Retrieve latest Google access token by user_id (UUID) from tokens table
    pub async fn get_latest_google_access_token_by_user_id(&self, user_id: uuid::Uuid) -> Result<Option<String>, AppError> {
        const GOOGLE_PROVIDER_ID: i16 = 2;
        const OAUTH_ACCESS_TOKEN_TYPE: i16 = 2;
        let query = r#"
            SELECT t.value
            FROM tokens t
            JOIN linked_accounts la ON la.id = t.linked_account_id
            WHERE la.user_id = $1
              AND la.provider_id = $2
              AND t.type_id = $3
              AND (t.expires_at IS NULL OR t.expires_at > NOW())
            ORDER BY t.expires_at DESC NULLS LAST
            LIMIT 1
        "#;
        match self.database.query_one(query, &[&user_id, &GOOGLE_PROVIDER_ID, &OAUTH_ACCESS_TOKEN_TYPE]).await {
            Ok(row) => Ok(Some(row.get(0))),
            Err(e) => {
                let is_no_rows = e.to_string().contains("no rows");
                if is_no_rows { Ok(None) } else { Err(AppError::DatabaseError(format!("Token lookup by user_id failed: {}", e))) }
            }
        }
    }

    /// Retrieve latest Google refresh token by user_id (UUID) from tokens table
    pub async fn get_latest_google_refresh_token_by_user_id(&self, user_id: uuid::Uuid) -> Result<Option<String>, AppError> {
        const GOOGLE_PROVIDER_ID: i16 = 2;
        const OAUTH_REFRESH_TOKEN_TYPE: i16 = 3;
        let query = r#"
            SELECT t.value
            FROM tokens t
            JOIN linked_accounts la ON la.id = t.linked_account_id
            WHERE la.user_id = $1
              AND la.provider_id = $2
              AND t.type_id = $3
              AND (t.expires_at IS NULL OR t.expires_at > NOW())
            ORDER BY t.expires_at DESC NULLS LAST
            LIMIT 1
        "#;
        match self.database.query_one(query, &[&user_id, &GOOGLE_PROVIDER_ID, &OAUTH_REFRESH_TOKEN_TYPE]).await {
            Ok(row) => Ok(Some(row.get(0))),
            Err(e) => {
                let is_no_rows = e.to_string().contains("no rows");
                if is_no_rows { Ok(None) } else { Err(AppError::DatabaseError(format!("Refresh token lookup by user_id failed: {}", e))) }
            }
        }
    }

    /// Update or insert a new access token after refresh
    pub async fn update_access_token(&self, user_id: uuid::Uuid, new_access_token: &str, expires_at: DateTime<Utc>) -> Result<(), AppError> {
        const GOOGLE_PROVIDER_ID: i16 = 2;
        const OAUTH_ACCESS_TOKEN_TYPE: i16 = 2;
        
        // First get the linked account ID
        let linked_account_query = r#"
            SELECT id FROM linked_accounts 
            WHERE user_id = $1 AND provider_id = $2
        "#;
        
        let linked_account_row = self.database.query_one(linked_account_query, &[&user_id, &GOOGLE_PROVIDER_ID]).await
            .map_err(|e| AppError::DatabaseError(format!("Failed to find linked account: {}", e)))?;
        
        let linked_account_id: uuid::Uuid = linked_account_row.get(0);
        
        // Use upsert to update or insert the access token
        let upsert_query = r#"
            INSERT INTO tokens (id, linked_account_id, type_id, value, expires_at, created_at)
            VALUES (gen_random_uuid(), $1, $2, $3, $4, NOW())
            ON CONFLICT (linked_account_id, type_id) 
            DO UPDATE SET value = EXCLUDED.value, expires_at = EXCLUDED.expires_at, created_at = NOW()
        "#;
        
        self.database.execute(upsert_query, &[&linked_account_id, &OAUTH_ACCESS_TOKEN_TYPE, &new_access_token, &expires_at]).await
            .map_err(|e| AppError::DatabaseError(format!("Failed to update access token: {}", e)))?;
        
        console_log!("🔄 Refreshed and stored new access token for user_id: {}", user_id);
        Ok(())
    }

    /// Development-only helper: fetch (masked) latest Google OAuth tokens + scopes for a user email
    /// WARNING: Do not expose in production. Caller must gate by environment.
    pub async fn dev_fetch_google_token_info(&self, user_email: &str) -> Result<Option<DevGoogleTokenInfo>, AppError> {
        const GOOGLE_PROVIDER_ID: i16 = 2;
        // We pull access + refresh tokens (latest) and scopes attached to the access token.
        let sql = r#"
            SELECT 
              la.provider_email,
              max(CASE WHEN t.type_id = 2 THEN t.value END)        AS access_token,
              max(CASE WHEN t.type_id = 2 THEN t.expires_at END)   AS access_expires_at,
              max(CASE WHEN t.type_id = 3 THEN t.value END)        AS refresh_token,
              array_agg(DISTINCT ts.scope) FILTER (WHERE ts.scope IS NOT NULL) AS scopes
            FROM users u
            JOIN linked_accounts la ON la.user_id = u.id AND la.provider_id = $2
            LEFT JOIN tokens t ON t.linked_account_id = la.id
            LEFT JOIN token_scopes ts ON ts.token_id = t.id AND t.type_id = 2
            WHERE u.primary_email ILIKE $1
            GROUP BY la.provider_email
            LIMIT 1
        "#;

        match self.database.query_one(sql, &[&user_email, &GOOGLE_PROVIDER_ID]).await {
            Ok(row) => {
                let provider_email: String = row.get("provider_email");
                let access_token: Option<String> = row.try_get("access_token").ok();
                let access_expires_at: Option<DateTime<Utc>> = row.try_get("access_expires_at").ok();
                let refresh_token: Option<String> = row.try_get("refresh_token").ok();
                let scopes: Option<Vec<Option<String>>> = row.try_get("scopes").ok();
                let scopes_flat: Vec<String> = scopes.unwrap_or_default().into_iter().flatten().collect();

                Ok(Some(DevGoogleTokenInfo::from_raw(
                    provider_email,
                    access_token,
                    access_expires_at,
                    refresh_token,
                    scopes_flat,
                )))
            }
            Err(e) => {
                let is_no_rows = e.to_string().contains("no rows");
                if is_no_rows { Ok(None) } else { Err(AppError::DatabaseError(format!("Failed to fetch dev token info: {}", e))) }
            }
        }
    }
}

/// Struct returned by dev_fetch_google_token_info (masked tokens)
#[derive(Debug, serde::Serialize)]
pub struct DevGoogleTokenInfo {
    pub email: String,
    pub has_access_token: bool,
    pub access_token_preview: Option<String>,
    pub access_expires_at: Option<String>,
    pub has_refresh_token: bool,
    pub refresh_token_preview: Option<String>,
    pub scopes: Vec<String>,
}

impl DevGoogleTokenInfo {
    fn mask(token: &str) -> String { if token.len() <= 12 { token.to_string() } else { format!("{}...{}", &token[..6], &token[token.len()-4..]) } }
    fn from_raw(email: String, access: Option<String>, access_exp: Option<DateTime<Utc>>, refresh: Option<String>, scopes: Vec<String>) -> Self {
        Self {
            email,
            has_access_token: access.is_some(),
            access_token_preview: access.as_ref().map(|t| Self::mask(t)),
            access_expires_at: access_exp.map(|dt| dt.to_rfc3339()),
            has_refresh_token: refresh.is_some(),
            refresh_token_preview: refresh.as_ref().map(|t| Self::mask(t)),
            scopes,
        }
    }
}