use serde_json::{json, Value};
use worker::console_log;
use crate::entity::user::User;
use uuid::Uuid;

/// Neon Database Client for HTTP API interactions
#[derive(Clone)]
pub struct NeonClient {
    pub project_id: String,
    pub database_name: String,
    pub connection_string: String,
    pub api_key: Option<String>,
}

impl NeonClient {
    /// Create a new Neon client instance
    pub fn new(connection_string: String) -> Self {
        // Extract project ID from connection string or use default
        let project_id = Self::extract_project_id(&connection_string)
            .unwrap_or_else(|| "fragrant-butterfly-56957862".to_string());
        
        let database_name = "neondb".to_string();
        
        Self {
            project_id,
            database_name,
            connection_string,
            api_key: None,
        }
    }

    /// Extract project ID from Neon connection string
    fn extract_project_id(connection_string: &str) -> Option<String> {
        // Parse connection string to extract project ID
        // Format: postgresql://user:pass@host/db?project=project_id
        if let Some(query_start) = connection_string.find('?') {
            let query_part = &connection_string[query_start + 1..];
            for param in query_part.split('&') {
                if let Some((key, value)) = param.split_once('=') {
                    if key == "project" {
                        return Some(value.to_string());
                    }
                }
            }
        }
        None
    }

    /// Execute SQL query against Neon database
    pub async fn execute_sql(&self, sql: &str) -> Result<Value, String> {
        console_log!("Executing SQL against Neon database: {}", sql);
        
        // For now, simulate the database operation
        // TODO: Implement actual HTTP request to Neon SQL API
        // This would involve making a POST request to the Neon API endpoint
        // with proper authentication and the SQL query
        
        // Simulate successful response for demo
        Ok(json!({
            "success": true,
            "project_id": self.project_id,
            "database": self.database_name,
            "sql": sql,
            "rows_affected": 1
        }))
    }

    /// Find user by username
    pub async fn find_user_by_username(&self, username: &str) -> Result<Option<User>, String> {
        console_log!("Looking up user in Neon database: {}", username);
        
        let sql = format!(
            "SELECT id, username, email, password_hash, created_at FROM users WHERE username = '{}';",
            username
        );
        
        match self.execute_sql(&sql).await {
            Ok(_result) => {
                // For now, simulate finding the demo user
                if username == "demo" {
                    let demo_password_hash = format!("hashed_{}", "password");
                    Ok(Some(User {
                        id: "demo-user-id".to_string(),
                        username: "demo".to_string(),
                        email: "demo@example.com".to_string(),
                        password_hash: demo_password_hash,
                        role: crate::entity::role_type::Role::User,
                        is_active: true,
                        is_verified: true,
                    }))
                } else {
                    Ok(None)
                }
            }
            Err(e) => Err(format!("Database query failed: {}", e))
        }
    }

    /// Find user by ID
    pub async fn find_user_by_id(&self, user_id: &str) -> Result<Option<User>, String> {
        console_log!("Looking up user by ID in Neon database: {}", user_id);
        
        let sql = format!(
            "SELECT id, username, email, password_hash, created_at FROM users WHERE id = '{}';",
            user_id
        );
        
        match self.execute_sql(&sql).await {
            Ok(_result) => {
                // For now, simulate finding the demo user
                if user_id == "demo-user-id" {
                    let demo_password_hash = format!("hashed_{}", "password");
                    Ok(Some(User {
                        id: "demo-user-id".to_string(),
                        username: "demo".to_string(),
                        email: "demo@example.com".to_string(),
                        password_hash: demo_password_hash,
                        role: crate::entity::role_type::Role::User,
                        is_active: true,
                        is_verified: true,
                    }))
                } else {
                    Ok(None)
                }
            }
            Err(e) => Err(format!("Database query failed: {}", e))
        }
    }

    /// Create a new user
    pub async fn create_user(&self, username: &str, email: &str, password_hash: &str) -> Result<User, String> {
        console_log!("Creating user in Neon database: {} with email: {}", username, email);
        
        let user_id = Uuid::new_v4().to_string();
        let created_at = js_sys::Date::new_0().to_iso_string().as_string().unwrap_or_default();
        
        let sql = format!(
            "INSERT INTO users (id, username, email, password_hash, created_at) VALUES ('{}', '{}', '{}', '{}', '{}') RETURNING id;",
            user_id, username, email, password_hash, created_at
        );
        
        match self.execute_sql(&sql).await {
            Ok(_result) => {
                let user = User {
                    id: user_id,
                    username: username.to_string(),
                    email: email.to_string(),
                    password_hash: password_hash.to_string(),
                    role: crate::entity::role_type::Role::User,
                    is_active: true,
                    is_verified: false,
                };
                
                console_log!("User created in Neon database successfully: {}", user.id);
                Ok(user)
            }
            Err(e) => Err(format!("Failed to create user in database: {}", e))
        }
    }
}
