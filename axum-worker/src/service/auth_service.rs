use crate::models::UserInfo;
use crate::service::user_service::UserService;
use crate::repo::user::UserRepository;

/// Authentication service that handles auth-specific business logic
#[derive(Clone)]
pub struct AuthService<R: UserRepository + Clone> {
    user_service: UserService<R>,
}

impl<R: UserRepository + Clone> AuthService<R> {
    pub fn new(user_service: UserService<R>) -> Self {
        Self { user_service }
    }

    // Note: Service initialization is handled by the repository's initialize method
    // which is called during application startup in state.rs

    /// Authenticate user and return user info if valid
    pub async fn authenticate_token(&self, token: &str) -> Result<UserInfo, String> {
        self.user_service.get_current_user(token).await
    }

    /// Get user service reference for delegation
    pub fn user_service(&self) -> &UserService<R> {
        &self.user_service
    }
}
