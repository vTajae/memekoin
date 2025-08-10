//! PostgreSQL session store implementation for axum-sessions
//! 
//! WASM-compatible session storage using chrono instead of std::time

use std::collections::HashMap;
use axum_sessions::{SessionStore, SessionData};
use chrono::{DateTime, Utc};
use serde_json::Value;
use uuid::Uuid;
use worker::console_log;

use crate::database::Database;

/// PostgreSQL-backed session store for axum-sessions
#[derive(Clone)]
pub struct PostgresSessionStore {
    database: Database,
}

impl PostgresSessionStore {
    /// Create a new PostgreSQL session store
    pub fn new(database: Database) -> Self {
        Self { database }
    }
}

#[async_trait::async_trait]
impl SessionStore for PostgresSessionStore {
    async fn load_session(&self, cookie_value: String) -> Result<Option<SessionData>, Box<dyn std::error::Error + Send + Sync>> {
        // Use session token as session ID
        let session_id = cookie_value;
        
        console_log!("🔐 AXUM-SESSIONS: Loading session: {}", session_id);
        
        // Query the sessions table using our repository format
        let query = "SELECT data, expires FROM sessions WHERE id = $1";
        
        match self.database.query_opt(query, &[&session_id]).await {
            Ok(Some(row)) => {
                let data_json: String = row.get("data");
                let expires: i64 = row.get("expires");
                
                // Check if session has expired using chrono
                let now_timestamp = Utc::now().timestamp();
                if expires <= now_timestamp {
                    console_log!("🔐 AXUM-SESSIONS: Session expired: {}", session_id);
                    // Clean up expired session
                    let _ = self.database.execute("DELETE FROM sessions WHERE id = $1", &[&session_id]).await;
                    return Ok(None);
                }
                
                // Decode session data
                let data: HashMap<String, Value> = serde_json::from_str(&data_json)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
                
                // Create SessionData with expiry time
                let expiry = DateTime::from_timestamp(expires, 0)
                    .ok_or("Invalid timestamp")?;
                
                let session_data = SessionData::new(data, Some(expiry));
                
                console_log!("🔐 AXUM-SESSIONS: Session loaded successfully: {}", session_id);
                Ok(Some(session_data))
            }
            Ok(None) => {
                console_log!("🔐 AXUM-SESSIONS: Session not found: {}", session_id);
                Ok(None)
            }
            Err(e) => {
                console_log!("🔐 AXUM-SESSIONS: Database error loading session: {}", e);
                Err(Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
            }
        }
    }

    async fn store_session(&self, session_data: SessionData) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // Generate a new session ID
        let session_id = Uuid::new_v4().to_string();
        
        console_log!("🔐 AXUM-SESSIONS: Storing session: {}", session_id);
        
        // Serialize session data
        let data_json = serde_json::to_string(session_data.data())
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        
        // Calculate expiry timestamp (default to 24 hours if not set)
        let expires = match session_data.expires() {
            Some(expiry) => expiry.timestamp(),
            None => (Utc::now() + chrono::Duration::hours(24)).timestamp(),
        };
        
        // Insert into sessions table using our repository format
        let query = "
            INSERT INTO sessions (id, data, expires) 
            VALUES ($1, $2, $3)
            ON CONFLICT (id) DO UPDATE SET 
                data = EXCLUDED.data,
                expires = EXCLUDED.expires
        ";
        
        self.database.execute(query, &[&session_id, &data_json, &expires]).await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        
        console_log!("🔐 AXUM-SESSIONS: Session stored successfully: {}", session_id);
        Ok(session_id)
    }

    async fn update_session(&self, cookie_value: String, session_data: SessionData) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let session_id = cookie_value;
        
        console_log!("🔐 AXUM-SESSIONS: Updating session: {}", session_id);
        
        // Serialize session data
        let data_json = serde_json::to_string(session_data.data())
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        
        // Calculate expiry timestamp
        let expires = match session_data.expires() {
            Some(expiry) => expiry.timestamp(),
            None => (Utc::now() + chrono::Duration::hours(24)).timestamp(),
        };
        
        // Update sessions table
        let query = "
            UPDATE sessions 
            SET data = $2, expires = $3
            WHERE id = $1
        ";
        
        let rows_affected = self.database.execute(query, &[&session_id, &data_json, &expires]).await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        
        if rows_affected == 0 {
            // Session doesn't exist, create it
            return self.store_session(session_data).await;
        }
        
        console_log!("🔐 AXUM-SESSIONS: Session updated successfully: {}", session_id);
        Ok(session_id)
    }

    async fn destroy_session(&self, cookie_value: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let session_id = cookie_value;
        
        console_log!("🔐 AXUM-SESSIONS: Destroying session: {}", session_id);
        
        // Delete from sessions table
        let query = "DELETE FROM sessions WHERE id = $1";
        
        self.database.execute(query, &[&session_id]).await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        
        console_log!("🔐 AXUM-SESSIONS: Session destroyed successfully: {}", session_id);
        Ok(())
    }
}