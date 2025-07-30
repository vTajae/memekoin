use crate::entity::role_type::Role;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use worker::console_log;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub role: Role,
    pub is_active: bool,
    pub is_verified: bool,
}

impl User {
    pub fn new(
        id: Uuid,
        email: String,
        username: String,
        password: String,
        role: Role,
        is_active: bool,
        is_verified: bool,
    ) -> Self {
        // Simple password hashing for demo - NOT production ready
        let password_hash = format!("hashed_{}", password);

        Self {
            id: id.to_string(),
            email,
            username,
            password_hash,
            role,
            is_active,
            is_verified,
        }
    }

    pub fn verify_password(&self, password: &str) -> bool {
        // Use SHA256 hashing to match the database setup script
        use sha2::{Sha256, Digest};

        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        let expected_hash = format!("{:x}", hasher.finalize());

        console_log!("LIVE DATABASE: Verifying password for user: {}", self.username);
        console_log!("LIVE DATABASE: Expected hash: {}", expected_hash);
        console_log!("LIVE DATABASE: Stored hash: {}", self.password_hash);

        let matches = self.password_hash == expected_hash;
        console_log!("LIVE DATABASE: Password verification result: {}", matches);
        matches
    }
}
