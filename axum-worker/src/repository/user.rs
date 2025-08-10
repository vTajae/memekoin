//! User repository - Database-backed user data access operations

use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    error::AppError,
    database::{Database, FromRow},
};
use crate::database::Row;

/// User entity aligned with database schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub password: Option<String>, // Hashed password for non-OAuth users
    pub created_at: DateTime<Utc>,
}

impl FromRow<User> for User {
    fn from_row(row: Row) -> Result<User, AppError> {
        Ok(User {
            id: row.get("id"),
            email: row.get("email"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            password: row.get("password"),
            created_at: row.get("created_at"),
        })
    }
}

impl User {
    /// Create new user for OAuth (no password)
    pub fn new_oauth(email: String, first_name: Option<String>, last_name: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            email,
            first_name,
            last_name,
            password: None,
            created_at: Utc::now(),
        }
    }

    /// Get user ID
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Get display name
    pub fn display_name(&self) -> String {
        match (&self.first_name, &self.last_name) {
            (Some(first), Some(last)) => format!("{} {}", first, last),
            (Some(first), None) => first.clone(),
            (None, Some(last)) => last.clone(),
            (None, None) => self.email.clone(),
        }
    }

    /// Check if user is OAuth user (no password)
    pub fn is_oauth_user(&self) -> bool {
        self.password.is_none()
    }
}

/// Database-backed user repository
pub struct UserRepository {
    database: Arc<Database>,
}

impl UserRepository {
    /// Create new user repository with database connection
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }

    /// Find user by email
    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let query = "SELECT id, email, first_name, last_name, password, created_at FROM users WHERE email = $1";
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] = &[&email];
        let row = self.database.query_opt(query, params).await?;
        
        match row {
            Some(r) => Ok(Some(User::from_row(r)?)),
            None => Ok(None),
        }
    }

    /// Find user by ID
    pub async fn find_by_id(&self, id: &Uuid) -> Result<Option<User>, AppError> {
        let query = "SELECT id, email, first_name, last_name, password, created_at FROM users WHERE id = $1";
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] = &[id];
        let row = self.database.query_opt(query, params).await?;
        
        match row {
            Some(r) => Ok(Some(User::from_row(r)?)),
            None => Ok(None),
        }
    }

    /// Create new user
    pub async fn create(&self, user: User) -> Result<User, AppError> {
        let query = r#"
            INSERT INTO users (id, email, first_name, last_name, password, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, email, first_name, last_name, password, created_at
        "#;
        
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] = &[
            &user.id,
            &user.email,
            &user.first_name,
            &user.last_name,
            &user.password,
            &user.created_at,
        ];
        
        let row = self.database.query_one(query, params).await?;
        User::from_row(row)
    }

    /// Update user information
    pub async fn update(&self, user: User) -> Result<User, AppError> {
        let query = r#"
            UPDATE users 
            SET email = $2, first_name = $3, last_name = $4, password = $5
            WHERE id = $1
            RETURNING id, email, first_name, last_name, password, created_at
        "#;
        
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] = &[
            &user.id,
            &user.email,
            &user.first_name,
            &user.last_name,
            &user.password,
        ];
        
        let row = self.database.query_one(query, params).await?;
        User::from_row(row)
    }

    /// Delete user
    pub async fn delete(&self, id: &Uuid) -> Result<bool, AppError> {
        let query = "DELETE FROM users WHERE id = $1";
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] = &[id];
        let affected = self.database.execute(query, params).await?;
        Ok(affected > 0)
    }

    /// Check if email exists
    pub async fn email_exists(&self, email: &str) -> Result<bool, AppError> {
        let query = "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)";
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] = &[&email];
        let row = self.database.query_one(query, params).await?;
        Ok(row.get(0))
    }

    /// Find or create user (for OAuth flow)
    pub async fn find_or_create_oauth_user(
        &self,
        email: String,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> Result<User, AppError> {
        // First try to find existing user
        if let Some(existing_user) = self.find_by_email(&email).await? {
            return Ok(existing_user);
        }

        // Create new user
        let new_user = User::new_oauth(email, first_name, last_name);
        self.create(new_user).await
    }
    
    /// Get user by email - alias for find_by_email (for handler compatibility)
    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<serde_json::Value>, AppError> {
        if let Some(user) = self.find_by_email(email).await? {
            let user_json = serde_json::json!({
                "id": user.id,
                "email": user.email,
                "first_name": user.first_name,
                "last_name": user.last_name,
                "password": user.password,
                "created_at": user.created_at.to_rfc3339()
            });
            Ok(Some(user_json))
        } else {
            Ok(None)
        }
    }
    
    /// Get user by ID - alias for find_by_id (for handler compatibility)
    pub async fn get_user_by_id(&self, id: &str) -> Result<Option<crate::entity::users::User>, AppError> {
        let uuid_id = uuid::Uuid::parse_str(id)
            .map_err(|_| AppError::ValidationError("Invalid user ID format".to_string()))?;
        
        if let Some(repo_user) = self.find_by_id(&uuid_id).await? {
            // Convert repository::User to entity::users::User
            let is_oauth = repo_user.is_oauth_user();
            let entity_user = crate::entity::users::User::new(
                repo_user.id,
                repo_user.email,
                repo_user.first_name,
                repo_user.last_name,
                None, // password not exposed
                is_oauth, // is_oauth
            );
            Ok(Some(entity_user))
        } else {
            Ok(None)
        }
    }
    
    /// Create OAuth user (for handler compatibility)
    pub async fn create_oauth_user(
        &self,
        email: &str,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> Result<serde_json::Value, AppError> {
        let user = self.find_or_create_oauth_user(email.to_string(), first_name, last_name).await?;
        let user_json = serde_json::json!({
            "id": user.id,
            "email": user.email,
            "first_name": user.first_name,
            "last_name": user.last_name,
            "created_at": user.created_at.to_rfc3339()
        });
        Ok(user_json)
    }
}

