use crate::models::{UserInfo, LoginRequest, RegisterRequest, AuthResponse};
use crate::repo::user::UserRepository;
use crate::auth::{hash_password, verify_password, create_jwt_token};
use worker::console_log;

/// User service containing business logic for user operations
#[derive(Clone)]
pub struct UserService<R: UserRepository + Clone> {
    user_repo: R,
    jwt_secret: String,
}

impl<R: UserRepository + Clone> UserService<R> {
    pub fn new(user_repo: R, jwt_secret: String) -> Self {
        Self {
            user_repo,
            jwt_secret,
        }
    }

    // Note: Service initialization is handled by the repository's initialize method
    // which is called during application startup in state.rs

    /// Register a new user
    pub async fn register(&self, request: RegisterRequest) -> Result<AuthResponse, String> {
        console_log!("Registering user: {}", request.username);
        
        // Hash password
        let password_hash = hash_password(&request.password).await?;
        
        // Create user in repository
        match self.user_repo.create(&request.username, &request.email, &password_hash).await {
            Ok(user) => {
                let token = create_jwt_token(&user, &self.jwt_secret).await?;
                let user_info = user.to_user_info();
                
                Ok(AuthResponse {
                    success: true,
                    message: "User registered successfully".to_string(),
                    user: Some(user_info),
                    token: Some(token),
                })
            }
            Err(e) => Ok(AuthResponse {
                success: false,
                message: format!("Registration failed: {}", e),
                user: None,
                token: None,
            })
        }
    }

    /// Login a user
    pub async fn login(&self, request: LoginRequest) -> Result<AuthResponse, String> {
        console_log!("Login attempt for user: {}", request.username);
        
        // Find user by username
        let user = match self.user_repo.find_by_username(&request.username).await? {
            Some(u) => u,
            None => return Ok(AuthResponse {
                success: false,
                message: "Invalid credentials".to_string(),
                user: None,
                token: None,
            })
        };

        // Verify password
        let password_valid = verify_password(&request.password, &user.password_hash).await
            .map_err(|e| format!("Password verification failed: {}", e))?;
            
        if !password_valid {
            return Ok(AuthResponse {
                success: false,
                message: "Invalid credentials".to_string(),
                user: None,
                token: None,
            });
        }

        // Create JWT token
        let token = create_jwt_token(&user, &self.jwt_secret).await?;
        let user_info = user.to_user_info();

        Ok(AuthResponse {
            success: true,
            message: "Login successful".to_string(),
            user: Some(user_info),
            token: Some(token),
        })
    }

    /// Get current user by token
    pub async fn get_current_user(&self, token: &str) -> Result<UserInfo, String> {
        use crate::auth::verify_jwt_token;
        
        let claims = verify_jwt_token(token, &self.jwt_secret).await?;
        
        let user = match self.user_repo.find_by_id(&claims.sub).await? {
            Some(u) => u,
            None => return Err("User not found".to_string())
        };

        Ok(user.to_user_info())
    }

    // Note: Demo user creation is handled by the repository's initialize method
}
