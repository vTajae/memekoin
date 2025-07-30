use serde::{Deserialize, Serialize};

/// Domain model for User - this is the canonical user representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: String,
}

/// User info for API responses (without sensitive data)
#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub email: String,
}

/// Login request structure
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Registration request structure
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// Authentication response
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub success: bool,
    pub message: String,
    pub user: Option<UserInfo>,
    pub token: Option<String>,
}

impl User {
    pub fn to_user_info(&self) -> UserInfo {
        UserInfo {
            id: self.id.clone(),
            username: self.username.clone(),
            email: self.email.clone(),
        }
    }
}
