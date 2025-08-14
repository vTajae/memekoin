//! Application State - Centralized dependency injection container

use std::sync::{Arc, OnceLock};
use worker::Env;

use crate::{
    utils::error::AppError,
    database::Database,
    repository::{
        user::UserRepository,
        session::SessionRepository,
    },
};

/// Helper: read a config value preferring a Cloudflare Secret, then falling back to a plain var
fn secret_or_var(env: &Env, key: &str) -> Option<String> {
    env
        .secret(key)
        .map(|s| s.to_string())
        .or_else(|_| env.var(key).map(|v| v.to_string()))
        .ok()
}

/// Application state containing all services and dependencies
#[derive(Clone)]
pub struct AppState {
    /// Database connection (eagerly initialized at startup)
    pub database: Arc<Database>,
    /// User repository (lazy singleton)
    user_repository: OnceLock<Arc<UserRepository>>,
    /// Session repository (lazy singleton)
    session_repository: OnceLock<Arc<SessionRepository>>,
    /// Google OAuth configuration
    pub google_oauth_config: GoogleOAuthConfig,
    /// Application configuration
    pub config: AppConfig,
}

/// Google OAuth configuration
#[derive(Clone)]
pub struct GoogleOAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub auth_url: String,
    pub token_url: String,
}

/// Application configuration
#[derive(Clone)]
pub struct AppConfig {
    pub environment: String,
    pub base_url: String,
    pub session_secret: String,
    pub database_url: String,
}

impl AppState {
    /// Initialize application state with environment variables (fully synchronous)
    pub async fn new(env: Env) -> Result<Self, AppError> {
        worker::console_log!("=== APPSTATE INITIALIZATION (SYNCHRONOUS) ===");
        
        // Extract configuration from environment
        let google_oauth_config = GoogleOAuthConfig {
            // Client ID isn't secret, but support secret store if provided
            client_id: secret_or_var(&env, "GOOGLE_CLIENT_ID")
                .unwrap_or("test-client-id".to_string()),
            // Prefer secret store for sensitive values
            client_secret: secret_or_var(&env, "GOOGLE_CLIENT_SECRET")
                .unwrap_or("test-client-secret".to_string()),
            redirect_uri: env
                .var("GOOGLE_REDIRECT_URI")
                .map(|v| v.to_string())
                .unwrap_or("http://localhost:8787/api/auth/oauth/callback".to_string()),
            auth_url: "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
            token_url: "https://oauth2.googleapis.com/token".to_string(),
        };

        let app_config = AppConfig {
            environment: env
                .var("ENVIRONMENT")
                .map(|v| v.to_string())
                .unwrap_or("development".to_string()),
            base_url: env
                .var("BASE_URL")
                .map(|v| v.to_string())
                .unwrap_or("http://localhost:8787".to_string()),
            // Prefer secret store for session secret
            session_secret: secret_or_var(&env, "SESSION_SECRET")
                .unwrap_or("development-secret-change-in-production".to_string()),
            database_url: secret_or_var(&env, "DEV_DATABASE_URL")                .unwrap_or("postgres://leptos_user:leptos_pass@localhost:5432/leptos_db".to_string()),
        };

    // Initialize database client (lenient for local dev): continue even if DB is unreachable
    worker::console_log!("DATABASE: Initializing core client...");
    let database = Arc::new(Database::from_url_or_stub(&app_config.database_url).await);
    worker::console_log!("DATABASE: Initialized (may be stub if connection failed)");

        worker::console_log!("APPSTATE: Core clients prepared");

        Ok(Self {
            database,
            user_repository: OnceLock::new(),
            session_repository: OnceLock::new(),
            google_oauth_config,
            config: app_config,
        })
    }

    /// Get database (already initialized)
    pub fn database(&self) -> Arc<Database> {
        self.database.clone()
    }

    /// Get user repository (lazy singleton)
    pub fn user_repo(&self) -> Arc<UserRepository> {
        self.user_repository
            .get_or_init(|| Arc::new(UserRepository::new(self.database.clone())))
            .clone()
    }

    /// Get session repository (lazy singleton)
    pub fn session_repo(&self) -> Arc<SessionRepository> {
        self.session_repository
            .get_or_init(|| Arc::new(SessionRepository::new(self.database.clone())))
            .clone()
    }

    /// Test database connection
    pub async fn test_database_connection(&self) -> Result<(), AppError> {
        self.database.test_connection().await
    }
}
