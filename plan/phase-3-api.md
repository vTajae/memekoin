# Phase 3: API Implementation

**Duration**: 2-3 days  
**Priority**: High  
**Prerequisites**: Phases 1 & 2 completed  
**Status**: Ready for implementation after Phase 2

## Overview

Build comprehensive REST API with proper error handling, middleware integration, and future Gmail API preparation. Focus on robust API standards and extensible architecture.

## API Architecture

### REST API Standards
```yaml
base_url: https://your-worker.workers.dev/api
content_type: application/json
authentication: Cookie-based sessions
error_format: Standardized JSON responses
versioning: Path-based (/api/v1/) for future versions
```

### API Response Standards
```rust
// Success Response Format
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

// Error Response Format  
#[derive(Serialize)]
pub struct ApiError {
    pub success: bool,
    pub error: ErrorDetails,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize)]
pub struct ErrorDetails {
    pub code: String,           // ERROR_CODE_FORMAT
    pub message: String,        // Human-readable message
    pub details: Option<String>, // Technical details
}
```

## Implementation Tasks

### 1. DTO Layer Enhancement

#### Authentication DTOs (`src/dto/auth.rs`)
```rust
// OAuth Login Request
#[derive(Deserialize)]
pub struct OAuthLoginParams {
    pub redirect_after: Option<String>,
}

// OAuth Callback Parameters
#[derive(Deserialize)]
pub struct OAuthCallbackParams {
    pub code: String,
    pub state: String,
    pub error: Option<String>,
    pub error_description: Option<String>,
}

// Token Refresh Response
#[derive(Serialize)]
pub struct TokenRefreshResponse {
    pub expires_in: u32,
    pub token_type: String,
}

// User Profile Response
#[derive(Serialize)]
pub struct UserProfileResponse {
    pub user: User,
    pub session_expires_at: chrono::DateTime<chrono::Utc>,
}
```

#### Standard Response DTOs (`src/dto/response.rs`)
```rust
impl<T> ApiResponse<T> {
    pub fn success(data: Option<T>, message: Option<&str>) -> Self {
        Self {
            success: true,
            data,
            message: message.map(|s| s.to_string()),
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn success_with_message(message: &str) -> ApiResponse<()> {
        Self::success(None, Some(message))
    }
}

impl ApiError {
    pub fn new(code: &str, message: &str, details: Option<String>) -> Self {
        Self {
            success: false,
            error: ErrorDetails {
                code: code.to_string(),
                message: message.to_string(),
                details,
            },
            timestamp: chrono::Utc::now(),
        }
    }
}
```

### 2. Enhanced Error Handling

#### Error Code Standards (`src/error.rs`)
```rust
impl AppError {
    pub fn error_code(&self) -> &'static str {
        match self {
            AppError::Auth(AuthError::InvalidCredentials) => "AUTH_INVALID_CREDENTIALS",
            AppError::Auth(AuthError::MissingToken) => "AUTH_MISSING_TOKEN", 
            AppError::Auth(AuthError::InvalidToken) => "AUTH_INVALID_TOKEN",
            AppError::Auth(AuthError::UserNotFound) => "AUTH_USER_NOT_FOUND",
            AppError::Auth(AuthError::Forbidden) => "AUTH_FORBIDDEN",
            AppError::Auth(AuthError::OAuthError(_)) => "AUTH_OAUTH_ERROR",
            AppError::Auth(AuthError::SessionError(_)) => "AUTH_SESSION_ERROR",
            AppError::Validation(_) => "VALIDATION_ERROR",
            AppError::Database(_) => "DATABASE_ERROR",
            AppError::ExternalService(_) => "EXTERNAL_SERVICE_ERROR",
            AppError::Config(_) => "CONFIG_ERROR",
            AppError::Internal(_) => "INTERNAL_SERVER_ERROR",
        }
    }
}

// Enhanced IntoResponse implementation
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = self.status_and_message();
        
        let body = Json(ApiError::new(
            self.error_code(),
            message,
            Some(self.to_string()),
        ));

        (status, body).into_response()
    }
}
```

### 3. Middleware Implementation

#### Authentication Middleware (`src/middleware/auth.rs`)
```rust
pub struct AuthMiddleware;

impl<S> Layer<S> for AuthMiddleware {
    type Service = AuthService<S>;

    fn layer(&self, service: S) -> Self::Service {
        AuthService { inner: service }
    }
}

pub struct AuthService<S> {
    inner: S,
}

impl<S, B> Service<Request<B>> for AuthService<S>
where
    S: Service<Request<B>, Response = Response> + Clone + Send + 'static,
    S::Future: Send,
    B: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        // Extract session cookie and validate
        // Add user context to request extensions
        // Handle authentication errors
    }
}
```

#### Enhanced CORS Middleware (`src/middleware/cors.rs`)
```rust
pub struct CorsLayer {
    allow_origins: Vec<String>,
    allow_methods: Vec<Method>,
    allow_headers: Vec<HeaderName>,
    allow_credentials: bool,
    max_age: Option<Duration>,
}

impl CorsLayer {
    pub fn new() -> Self {
        Self {
            allow_origins: vec![
                "http://localhost:8787".to_string(),
                "https://your-domain.com".to_string(),
            ],
            allow_methods: vec![Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS],
            allow_headers: vec![
                HeaderName::from_static("content-type"),
                HeaderName::from_static("authorization"),
                HeaderName::from_static("cookie"),
            ],
            allow_credentials: true,
            max_age: Some(Duration::from_secs(3600)),
        }
    }
}
```

#### Request Logging Middleware (`src/middleware/logging.rs`)
```rust
pub struct LoggingLayer;

impl<S> Layer<S> for LoggingLayer {
    type Service = LoggingService<S>;

    fn layer(&self, service: S) -> Self::Service {
        LoggingService { inner: service }
    }
}

// Log request/response details for monitoring and debugging
impl<S, B> Service<Request<B>> for LoggingService<S> {
    // Implementation logs:
    // - Request method, path, headers (sanitized)
    // - Response status, duration
    // - Error details (if any)
    // - User context (if authenticated)
}
```

### 4. Complete API Endpoint Implementation

#### Enhanced Auth Routes (`src/routes/auth.rs`)
```rust
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/oauth/login", get(handler::auth::oauth_login))
        .route("/oauth/callback", get(handler::auth::oauth_callback))
        .route("/refresh", post(handler::auth::refresh_token))
        .route("/logout", post(handler::auth::logout))
        .route("/user", get(handler::auth::get_current_user))
        .route("/sessions", get(handler::auth::get_user_sessions))
        .route("/sessions/:session_id", delete(handler::auth::end_session))
        .layer(AuthMiddleware) // Protect endpoints that require authentication
}
```

#### Health Check Routes (`src/routes/health.rs`)
```rust
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(handler::health::basic_health))
        .route("/health/detailed", get(handler::health::detailed_health))
}

// Enhanced health checks
impl HealthHandler {
    pub async fn detailed_health(State(state): State<AppState>) -> Result<Json<ApiResponse<HealthStatus>>, AppError> {
        let health_status = HealthStatus {
            status: "healthy".to_string(),
            timestamp: chrono::Utc::now(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            environment: state.config.environment.clone(),
            services: ServiceHealth {
                database: check_database_connection().await,
                oauth: check_oauth_configuration(&state.google_oauth_config),
                session_store: check_session_store_health().await,
            },
        };

        Ok(Json(ApiResponse::success(Some(health_status), None)))
    }
}
```

### 5. Future Gmail API Preparation

#### Gmail API Structure (`src/routes/gmail.rs`)
```rust
// Prepare structure for future Gmail integration
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/messages", get(handler::gmail::list_messages))
        .route("/messages/:message_id", get(handler::gmail::get_message))
        .route("/messages/send", post(handler::gmail::send_message))
        .route("/labels", get(handler::gmail::list_labels))
        .layer(AuthMiddleware) // Require authentication for all Gmail endpoints
}

// Gmail DTOs for future implementation
#[derive(Serialize, Deserialize)]
pub struct GmailMessage {
    pub id: String,
    pub thread_id: String,
    pub label_ids: Vec<String>,
    pub snippet: String,
    pub payload: MessagePayload,
    pub size_estimate: u32,
    pub history_id: String,
    pub internal_date: String,
}
```

### 6. Enhanced Route Configuration

#### Master Router Update (`src/routes/mod.rs`)
```rust
pub async fn create_router() -> Result<Router<AppState>, crate::error::AppError> {
    // Session store configuration
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(true)  // Enable in production
        .with_http_only(true)
        .with_same_site(tower_cookies::SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(Duration::hours(24)));

    Ok(Router::new()
        // API versioning preparation
        .nest("/v1", api_v1_routes())
        // Legacy routes (current)
        .merge(health::routes())
        .merge(auth::routes())
        // Future Gmail routes
        .merge(gmail::routes())
        // Global fallback
        .fallback(enhanced_fallback_handler)
        // Middleware stack (order matters)
        .layer(
            ServiceBuilder::new()
                .layer(LoggingLayer)           // Request logging first
                .layer(CookieManagerLayer::new()) // Cookie handling
                .layer(session_layer)          // Session management
                .layer(CorsLayer::new())       // CORS handling last
                .into_inner(),
        ))
}

pub fn api_v1_routes() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth::routes())
        .nest("/gmail", gmail::routes())
        .nest("/health", health::routes())
}
```

## Implementation Checklist

### DTO Layer
- [ ] Implement complete authentication DTOs
- [ ] Create standard response format DTOs
- [ ] Add validation attributes to request DTOs
- [ ] Include comprehensive serialization tests

### Error Handling
- [ ] Add error codes for all error types
- [ ] Enhance IntoResponse with detailed error info
- [ ] Include error logging and monitoring
- [ ] Test error response formats

### Middleware
- [ ] Implement authentication middleware
- [ ] Enhance CORS middleware with configuration
- [ ] Add request logging middleware
- [ ] Include middleware integration tests

### API Endpoints
- [ ] Complete authentication endpoint handlers
- [ ] Implement detailed health check endpoints
- [ ] Prepare Gmail API route structure
- [ ] Add comprehensive endpoint documentation

### Route Configuration
- [ ] Enhance master router with middleware stack
- [ ] Prepare API versioning structure
- [ ] Include proper error handling and fallbacks
- [ ] Test route resolution and middleware order

### Security & Validation
- [ ] Input validation for all endpoints
- [ ] Rate limiting preparation
- [ ] Request sanitization
- [ ] Security headers implementation

## Testing Strategy

### API Testing
```bash
# Authentication flow testing
curl -X GET "https://your-worker.workers.dev/api/auth/oauth/login"
curl -X GET "https://your-worker.workers.dev/api/auth/user" -H "Cookie: session_id=test"

# Health check testing
curl -X GET "https://your-worker.workers.dev/api/health"
curl -X GET "https://your-worker.workers.dev/api/health/detailed"

# Error handling testing
curl -X GET "https://your-worker.workers.dev/api/nonexistent"
```

### Integration Tests
```rust
#[tokio::test]
async fn test_authentication_flow() {
    // Test complete OAuth flow
}

#[tokio::test] 
async fn test_error_handling() {
    // Test error response formats
}

#[tokio::test]
async fn test_middleware_integration() {
    // Test middleware stack
}
```

## Security Standards

✅ **Input Validation** - Sanitize all request parameters  
✅ **Error Handling** - No sensitive information in error responses  
✅ **Security Headers** - Comprehensive security header implementation  
✅ **Rate Limiting** - Prepare for rate limiting implementation  
✅ **Authentication** - Proper session validation on protected endpoints  
✅ **CORS** - Restrictive CORS policy with credential support

## Success Criteria

✅ **Complete API** - All authentication endpoints functional  
✅ **Error Handling** - Comprehensive error response system  
✅ **Middleware** - Proper middleware stack integration  
✅ **Security** - Security best practices implemented  
✅ **Documentation** - API documentation and testing  
✅ **Extensibility** - Ready for Gmail API integration

## Next Phase

Once Phase 3 is complete, proceed to [Phase 4: Frontend Integration](./phase-4-frontend.md)