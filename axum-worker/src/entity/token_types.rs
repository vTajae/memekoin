//! Token Types Entity - Enhanced with business logic following schema v11
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TokenType {
    pub(crate) id: i16,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) expiration: String,
    pub(crate) created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TokenTypeVariant {
    AccessToken,
    RefreshToken,
    ApiKey,
    SessionToken,
    ResetToken,
    VerificationToken,
    InviteToken,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TokenExpiration {
    Never,
    Minutes(i32),
    Hours(i32),
    Days(i32),
    Weeks(i32),
    Months(i32),
    Custom(String),
}

impl TokenExpiration {
    pub fn to_string(&self) -> String {
        match self {
            TokenExpiration::Never => "never".to_string(),
            TokenExpiration::Minutes(m) => format!("{}m", m),
            TokenExpiration::Hours(h) => format!("{}h", h),
            TokenExpiration::Days(d) => format!("{}d", d),
            TokenExpiration::Weeks(w) => format!("{}w", w),
            TokenExpiration::Months(m) => format!("{}M", m),
            TokenExpiration::Custom(s) => s.clone(),
        }
    }

    pub fn to_seconds(&self) -> Option<i64> {
        match self {
            TokenExpiration::Never => None,
            TokenExpiration::Minutes(m) => Some(*m as i64 * 60),
            TokenExpiration::Hours(h) => Some(*h as i64 * 3600),
            TokenExpiration::Days(d) => Some(*d as i64 * 86400),
            TokenExpiration::Weeks(w) => Some(*w as i64 * 604800),
            TokenExpiration::Months(m) => Some(*m as i64 * 2592000), // Approximate 30 days
            TokenExpiration::Custom(_) => None, // Cannot determine without parsing
        }
    }
}

impl FromStr for TokenExpiration {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "never" {
            return Ok(TokenExpiration::Never);
        }

        if s.len() < 2 {
            return Err("Invalid expiration format".to_string());
        }

        let (number_part, unit_part) = s.split_at(s.len() - 1);
        let number: i32 = number_part.parse()
            .map_err(|_| "Invalid number in expiration".to_string())?;

        match unit_part {
            "m" => Ok(TokenExpiration::Minutes(number)),
            "h" => Ok(TokenExpiration::Hours(number)),
            "d" => Ok(TokenExpiration::Days(number)),
            "w" => Ok(TokenExpiration::Weeks(number)),
            "M" => Ok(TokenExpiration::Months(number)),
            _ => Ok(TokenExpiration::Custom(s.to_string())),
        }
    }
}

impl TokenTypeVariant {
    pub fn as_str(&self) -> &str {
        match self {
            TokenTypeVariant::AccessToken => "access_token",
            TokenTypeVariant::RefreshToken => "refresh_token",
            TokenTypeVariant::ApiKey => "api_key",
            TokenTypeVariant::SessionToken => "session_token",
            TokenTypeVariant::ResetToken => "reset_token",
            TokenTypeVariant::VerificationToken => "verification_token",
            TokenTypeVariant::InviteToken => "invite_token",
            TokenTypeVariant::Custom(name) => name,
        }
    }

    pub fn default_expiration(&self) -> TokenExpiration {
        match self {
            TokenTypeVariant::AccessToken => TokenExpiration::Hours(1),
            TokenTypeVariant::RefreshToken => TokenExpiration::Days(30),
            TokenTypeVariant::ApiKey => TokenExpiration::Never,
            TokenTypeVariant::SessionToken => TokenExpiration::Hours(24),
            TokenTypeVariant::ResetToken => TokenExpiration::Hours(1),
            TokenTypeVariant::VerificationToken => TokenExpiration::Hours(24),
            TokenTypeVariant::InviteToken => TokenExpiration::Days(7),
            TokenTypeVariant::Custom(_) => TokenExpiration::Hours(1),
        }
    }

    pub fn description(&self) -> String {
        match self {
            TokenTypeVariant::AccessToken => "Short-lived token for API access".to_string(),
            TokenTypeVariant::RefreshToken => "Long-lived token for renewing access tokens".to_string(),
            TokenTypeVariant::ApiKey => "Permanent API key for service access".to_string(),
            TokenTypeVariant::SessionToken => "Session token for web authentication".to_string(),
            TokenTypeVariant::ResetToken => "Token for password reset functionality".to_string(),
            TokenTypeVariant::VerificationToken => "Token for email/account verification".to_string(),
            TokenTypeVariant::InviteToken => "Token for user invitations".to_string(),
            TokenTypeVariant::Custom(name) => format!("Custom token type: {}", name),
        }
    }
}

impl TokenType {
    /// Create new token type
    pub(crate) fn new(
        id: i16,
        variant: TokenTypeVariant,
        custom_description: Option<String>,
        expiration: Option<TokenExpiration>,
    ) -> Self {
        let name = variant.as_str().to_string();
        let description = custom_description
            .unwrap_or_else(|| variant.description());
        let exp_str = expiration
            .unwrap_or_else(|| variant.default_expiration())
            .to_string();
        
        info!("Creating new token type: {} with expiration: {}", name, exp_str);
        
        Self {
            id,
            name,
            description,
            expiration: exp_str,
            created_at: chrono::Utc::now(),
        }
    }

    /// Create standard token types
    pub(crate) fn access_token(id: i16) -> Self {
        Self::new(id, TokenTypeVariant::AccessToken, None, None)
    }

    pub(crate) fn refresh_token(id: i16) -> Self {
        Self::new(id, TokenTypeVariant::RefreshToken, None, None)
    }

    pub(crate) fn api_key(id: i16) -> Self {
        Self::new(id, TokenTypeVariant::ApiKey, None, None)
    }

    pub(crate) fn session_token(id: i16) -> Self {
        Self::new(id, TokenTypeVariant::SessionToken, None, None)
    }

    /// Parse expiration string to TokenExpiration
    pub(crate) fn get_expiration(&self) -> Result<TokenExpiration, String> {
        TokenExpiration::from_str(&self.expiration)
    }

    /// Get expiration in seconds (None for never-expiring tokens)
    pub(crate) fn expiration_seconds(&self) -> Option<i64> {
        self.get_expiration()
            .ok()
            .and_then(|exp| exp.to_seconds())
    }

    /// Check if tokens of this type expire
    pub(crate) fn expires(&self) -> bool {
        self.expiration != "never"
    }

    /// Update expiration policy
    pub(crate) fn update_expiration(&mut self, new_expiration: TokenExpiration) {
        let new_exp_str = new_expiration.to_string();
        info!("Updating expiration for token type {} from {} to {}", 
              self.name, self.expiration, new_exp_str);
        self.expiration = new_exp_str;
    }

    /// Update description
    pub(crate) fn update_description(&mut self, new_description: String) {
        info!("Updating description for token type {}", self.name);
        self.description = new_description;
    }

    /// Get token type variant from name
    pub(crate) fn get_variant(&self) -> TokenTypeVariant {
        match self.name.as_str() {
            "access_token" => TokenTypeVariant::AccessToken,
            "refresh_token" => TokenTypeVariant::RefreshToken,
            "api_key" => TokenTypeVariant::ApiKey,
            "session_token" => TokenTypeVariant::SessionToken,
            "reset_token" => TokenTypeVariant::ResetToken,
            "verification_token" => TokenTypeVariant::VerificationToken,
            "invite_token" => TokenTypeVariant::InviteToken,
            _ => TokenTypeVariant::Custom(self.name.clone()),
        }
    }

    /// Calculate expiry timestamp from now
    pub(crate) fn calculate_expiry(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.expiration_seconds()
            .map(|seconds| chrono::Utc::now() + chrono::Duration::seconds(seconds))
    }

    /// Check if this is a security-sensitive token type
    pub(crate) fn is_sensitive(&self) -> bool {
        matches!(self.get_variant(), 
            TokenTypeVariant::AccessToken | 
            TokenTypeVariant::RefreshToken | 
            TokenTypeVariant::ResetToken |
            TokenTypeVariant::VerificationToken
        )
    }

    // Public accessors
    pub fn id(&self) -> i16 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn expiration(&self) -> &str {
        &self.expiration
    }

    pub fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.created_at
    }
}
