# State, Layering, and DI in Axum Worker

This document captures the architecture decision for dependency management, layering, and environment configuration.

**Last Updated**: December 11, 2024  
**Status**: ✅ IMPLEMENTED - Clean Architecture with Repository Pattern

## Goal
Modular, testable, and secure application architecture with clean data flow:

Route → Handler → Service → Repository → Database Client

## Decision Summary
- AppState owns only configuration and low-level clients.
  - AppConfig (includes `database_url`), GoogleOAuthConfig, Env
  - Database client (constructed from `database_url`)
- Repositories are exposed as lazy singletons (OnceLock) through non-optional getters.
- Services are not stored in AppState. Handlers/middleware construct services on demand or via thin factory methods.
- Environment resolution is centralized in AppState, preferring secrets.
  - `DEV_DATABASE_URL` secret → var → `DATABASE_URL` secret → var

## Why
- Avoids overlapping initialization of repositories/services across layers.
- Keeps state small and deterministic; improves testability.
- Mirrors the proven Controller/Service/Repository separation (Java reference) while fitting Rust ergonomics.
- Prevents tight coupling between AppState and service lifecycles (seen in small examples like rusty-worker).

## Implementation Highlights

### Current Implementation (December 2024)
- **Clean Architecture**: Complete separation of concerns with Handler → Service → Repository → Database layers
- **AuthRepository**: Centralized repository for all authentication-related database operations
  - User management (create, find, update)
  - Linked accounts for OAuth providers
  - Session storage and validation
  - Token management (access and refresh tokens)
- **AuthService**: Orchestrates authentication business logic
  - OAuth flow handling with Google
  - Session lifecycle management
  - Token validation and refresh
- **SessionService**: Manages user sessions
  - Session creation and validation
  - Cookie management
  - Session cleanup and expiration

### Repository Pattern Benefits
- **Testability**: Services can be tested with mock repositories
- **Maintainability**: Database changes isolated to repository layer
- **Scalability**: Easy to add new features without affecting existing code
- **Type Safety**: Strongly typed entities and DTOs throughout

### AppState Design
- Added `Database::from_url(&str)`; AppState resolves URL and constructs the DB client.
- AppState stores only database connection and configuration
- Services created on-demand by handlers:
  - `AuthService::new(database.clone())`
  - `SessionService::new(database.clone())`
- Repositories created within services as needed

## Security
- Secrets preferred over plain env vars for DB URLs.
- Session cookie should be HttpOnly; SameSite=Lax (or Strict) and Secure in production.
- Avoid logging sensitive values; truncate when necessary.

## Usage Examples

### Current Implementation Pattern
```rust
// In a handler - Clean Architecture flow
pub async fn handle_oauth_token(
    State(state): State<AppState>,
    Json(data): Json<FrontendOAuthSubmission>,
) -> Result<impl IntoResponse, AppError> {
    // Handler creates service
    let auth_service = AuthService::new(state.database.clone());
    
    // Service handles business logic
    let response = auth_service.handle_frontend_oauth(data).await?;
    
    // Return response
    Ok(Json(response))
}

// In AuthService - Service uses Repository
impl AuthService {
    pub fn new(database: Arc<Database>) -> Self {
        Self {
            auth_repository: AuthRepository::new(database),
        }
    }
    
    pub async fn handle_frontend_oauth(&self, data: FrontendOAuthSubmission) -> Result<OAuthResponse, AppError> {
        // Service orchestrates business logic
        // Repository handles database operations
        let user = self.auth_repository.create_or_find_user(...).await?;
        let session = self.auth_repository.create_session(...).await?;
        // ...
    }
}
```

### Layer Responsibilities
- **Handler**: HTTP concerns, request/response mapping
- **Service**: Business logic, orchestration, validation
- **Repository**: Database operations, query building
- **Database**: Connection management, query execution

## References
- Java (Controller → Service → Repository): https://github.com/vTajae/JavaFullStack
- Rust worker example: https://github.com/vTajae/rusty-worker/blob/main/src/handler/auth.rs

## Migration Notes
- Replace `state.user_repository()`/`state.session_repository()` Option getters with non-optional `state.user_repo()`/`state.session_repo()`.
- Construct services on demand; do not store services in AppState.
- Centralize DB URL resolution in AppState; pass to `Database::from_url`.
