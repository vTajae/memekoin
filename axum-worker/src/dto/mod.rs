//! Data Transfer Objects module

pub mod auth;
pub mod oauth;
pub mod response;

// Re-export specific types to avoid conflicts
pub use oauth::{OAuthState, GoogleTokenResponse, GoogleUserInfo};
pub use auth::{UserProfileResponse, UserResponse, UserSessionsResponse};
pub use response::*;