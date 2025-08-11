//! Session repository - Database-backed session management

use std::sync::Arc;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use serde_json;

use crate::{
    utils::error::AppError,
    database::{Database, FromRow, Row},
};

/// Session entity aligned with account_sessions table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub data: String,
    pub expires: i64, // Unix timestamp
    pub created_at: DateTime<Utc>,
}

impl FromRow<Session> for Session {
    fn from_row(row: Row) -> Result<Session, AppError> {
        Ok(Session {
            id: row.get("id"),
            data: row.get("data"),
            expires: row.get("expires"),
            created_at: Utc::now(), // Database doesn't store created_at, use current time
        })
    }
}

impl Session {
    /// Create new session with TTL in seconds
    pub fn new(id: String, data: String, ttl_seconds: i64) -> Self {
        let now = Utc::now();
        let expires = now.timestamp() + ttl_seconds;
        
        Self {
            id,
            data,
            expires,
            created_at: now,
        }
    }

    /// Check if session is expired
    pub fn is_expired(&self) -> bool {
        Utc::now().timestamp() > self.expires
    }

    /// Get remaining TTL in seconds
    pub fn remaining_ttl(&self) -> i64 {
        std::cmp::max(0, self.expires - Utc::now().timestamp())
    }

    /// Get expires as DateTime
    pub fn expires_at(&self) -> DateTime<Utc> {
        DateTime::from_timestamp(self.expires, 0).unwrap_or(Utc::now())
    }
}

/// User session data that gets serialized into session.data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSessionData {
    pub user_id: String,
    pub user_email: String,
    pub user_name: String,
    pub oauth_provider: String,
    pub access_token: Option<String>, // OAuth access token (encrypted)
    pub refresh_token: Option<String>, // OAuth refresh token (encrypted)
    pub token_expires_at: Option<DateTime<Utc>>,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
}

/// Database-backed session repository
pub struct SessionRepository {
    database: Arc<Database>,
}

impl SessionRepository {
    /// Create new session repository
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }

    /// Create new session
    pub async fn create(&self, session: Session) -> Result<Session, AppError> {
        let query = r#"
            INSERT INTO account_sessions (id, data, expires)
            VALUES ($1, $2, $3)
            ON CONFLICT (id) DO UPDATE SET
                data = EXCLUDED.data,
                expires = EXCLUDED.expires
            RETURNING id, data, expires
        "#;

        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] = &[&session.id, &session.data, &session.expires];
        let row = self.database.query_one(query, params).await?;

        Session::from_row(row)
    }

    /// Get session by ID
    pub async fn get(&self, session_id: &str) -> Result<Option<Session>, AppError> {
        let query = "SELECT id, data, expires FROM account_sessions WHERE id = $1";
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] = &[&session_id];
        let row = self.database.query_opt(query, params).await?;

        match row {
            Some(r) => {
                let session = Session::from_row(r)?;
                if session.is_expired() {
                    // Delete expired session
                    self.delete(session_id).await?;
                    Ok(None)
                } else {
                    Ok(Some(session))
                }
            }
            None => Ok(None),
        }
    }

    /// Update session data
    pub async fn update(&self, session_id: &str, data: String, ttl_seconds: i64) -> Result<Session, AppError> {
        let expires = Utc::now().timestamp() + ttl_seconds;
        let query = r#"
            UPDATE account_sessions
            SET data = $2, expires = $3
            WHERE id = $1
            RETURNING id, data, expires
        "#;

        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] = &[&session_id, &data, &expires];
        let row = self.database.query_one(query, params).await?;
        Session::from_row(row)
    }

    /// Delete session
    pub async fn delete(&self, session_id: &str) -> Result<bool, AppError> {
        let query = "DELETE FROM account_sessions WHERE id = $1";
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] = &[&session_id];
        let affected = self.database.execute(query, params).await?;
        Ok(affected > 0)
    }

    /// Cleanup expired sessions
    pub async fn cleanup_expired(&self) -> Result<u64, AppError> {
        let now = Utc::now().timestamp();
        let query = "DELETE FROM account_sessions WHERE expires < $1";
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] = &[&now];
        self.database.execute(query, params).await
    }

    /// Get all sessions (for maintenance)
    pub async fn get_all_active(&self) -> Result<Vec<Session>, AppError> {
        let now = Utc::now().timestamp();
        let query = "SELECT id, data, expires FROM account_sessions WHERE expires > $1 ORDER BY expires DESC";
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] = &[&now];
        let rows = self.database.query(query, params).await?;

        let mut sessions = Vec::new();
        for row in rows {
            sessions.push(Session::from_row(row)?);
        }
        Ok(sessions)
    }

    /// Create user session with OAuth data
    pub async fn create_user_session(
        &self,
        session_id: String,
        user_session_data: UserSessionData,
        ttl_seconds: i64,
    ) -> Result<Session, AppError> {
        let session_data = serde_json::to_string(&user_session_data)
            .map_err(|e| AppError::SerializationError(format!("Failed to serialize session data: {}", e)))?;
        
        let session = Session::new(session_id, session_data, ttl_seconds);
        self.create(session).await
    }

    /// Get user session data
    pub async fn get_user_session(&self, session_id: &str) -> Result<Option<UserSessionData>, AppError> {
        if let Some(session) = self.get(session_id).await? {
            let user_data: UserSessionData = serde_json::from_str(&session.data)
                .map_err(|e| AppError::SerializationError(format!("Failed to deserialize session data: {}", e)))?;
            Ok(Some(user_data))
        } else {
            Ok(None)
        }
    }

    /// Update user session data
    pub async fn update_user_session(
        &self,
        session_id: &str,
        user_session_data: UserSessionData,
        ttl_seconds: i64,
    ) -> Result<Session, AppError> {
        let session_data = serde_json::to_string(&user_session_data)
            .map_err(|e| AppError::SerializationError(format!("Failed to serialize session data: {}", e)))?;
        
        self.update(session_id, session_data, ttl_seconds).await
    }


    /// Legacy method for compatibility - Create session for user
    pub async fn create_for_user(&self, user_id: String, ttl_seconds: i64) -> Result<String, AppError> {
        let session_id = Uuid::new_v4().to_string();
        let user_data = UserSessionData {
            user_id: user_id.clone(),
            user_email: "".to_string(),
            user_name: "".to_string(),
            oauth_provider: "google".to_string(),
            access_token: None,
            refresh_token: None,
            token_expires_at: None,
            user_agent: None,
            ip_address: None,
        };
        
        self.create_user_session(session_id.clone(), user_data, ttl_seconds).await?;
        Ok(session_id)
    }

    /// Legacy method for compatibility - Find session by ID (returns basic Session)
    pub async fn find_by_id(&self, session_id: &str) -> Result<Option<Session>, AppError> {
        self.get(session_id).await
    }

    // Additional methods needed by SessionService
    
    /// Alias for create - needed by SessionService
    pub async fn create_session(&self, session: crate::entity::UserSession) -> Result<crate::entity::UserSession, AppError> {
        // Convert UserSession to Session for storage
        let session_data = serde_json::json!({
            "user_id": session.user_id(),
            "expires_at": session.expires_at(),
            "user_agent": session.user_agent(),
            "ip_address": session.ip_address()
        });
        
        let db_session = Session::new(
            session.session_id().to_string(),
            session_data.to_string(),
            (session.expires_at() - chrono::Utc::now()).num_seconds()
        );
        
        self.create(db_session).await?;
        Ok(session)
    }
    
    /// Get session by ID - alias for get
    pub async fn get_session(&self, session_id: &str) -> Result<Option<crate::entity::UserSession>, AppError> {
        if let Some(session) = self.get(session_id).await? {
            // Convert Session back to UserSession
            let session_data: serde_json::Value = serde_json::from_str(&session.data)
                .map_err(|e| AppError::SerializationError(format!("Failed to parse session data: {}", e)))?;
            
            let user_id = session_data["user_id"].as_str()
                .and_then(|s| uuid::Uuid::parse_str(s).ok())
                .ok_or_else(|| AppError::SerializationError("Invalid user_id in session".to_string()))?;
                
            let expires_at = session_data["expires_at"].as_str()
                .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .ok_or_else(|| AppError::SerializationError("Invalid expires_at in session".to_string()))?;
            
            let user_session = crate::entity::UserSession::new(
                session_id.to_string(),
                user_id,
                expires_at,
                session_data["user_agent"].as_str().map(|s| s.to_string()),
                session_data["ip_address"].as_str().map(|s| s.to_string()),
            );
            
            Ok(Some(user_session))
        } else {
            Ok(None)
        }
    }
    
    /// Update session - alias for update_user_session
    pub async fn update_session(&self, session_id: &str, session: crate::entity::UserSession) -> Result<(), AppError> {
        // Convert UserSession to Session data for storage
        let session_data = serde_json::json!({
            "user_id": session.user_id(),
            "expires_at": session.expires_at(),
            "user_agent": session.user_agent(),
            "ip_address": session.ip_address()
        });
        
        self.update(
            session_id, 
            session_data.to_string(),
            (session.expires_at() - chrono::Utc::now()).num_seconds()
        ).await?;
        
        Ok(())
    }
    
    /// Delete session by ID
    pub async fn delete_session(&self, session_id: &str) -> Result<(), AppError> {
        self.delete(session_id).await?;
        Ok(())
    }
    
    /// Get all sessions for a user
    pub async fn get_user_sessions(&self, user_id: &str) -> Result<Vec<crate::entity::UserSession>, AppError> {
        let query = "SELECT id, data, expires FROM account_sessions WHERE data::json->>'user_id' = $1";
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] = &[&user_id];
        let rows = self.database.query(query, params).await?;
        
        let mut sessions = Vec::new();
        for row in rows {
            if let Some(user_session) = self.get_session(&row.get::<_, String>("id")).await? {
                sessions.push(user_session);
            }
        }
        
        Ok(sessions)
    }
    
    /// Cleanup expired sessions
    pub async fn cleanup_expired_sessions(&self) -> Result<u32, AppError> {
        let count = self.cleanup_expired().await?;
        Ok(count as u32)
    }
}

