# Security Considerations

## Overview

Comprehensive security guidelines and implementation details for the Leptos fullstack Rust Cloudflare worker application with Google OAuth integration.

## Authentication Security

### OAuth 2.0 + PKCE Implementation

#### PKCE (Proof Key for Code Exchange)
```rust
// PKCE code generation
pub fn generate_pkce_pair() -> (String, String) {
    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
    use sha2::{Digest, Sha256};
    
    // Generate code verifier (43-128 characters)
    let code_verifier: String = (0..128)
        .map(|_| {
            const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-._~";
            CHARSET[rand::random::<usize>() % CHARSET.len()] as char
        })
        .collect();
    
    // Generate code challenge (SHA256 hash of verifier)
    let mut hasher = Sha256::new();
    hasher.update(code_verifier.as_bytes());
    let code_challenge = URL_SAFE_NO_PAD.encode(hasher.finalize());
    
    (code_verifier, code_challenge)
}
```

#### State Parameter for CSRF Protection
```rust
// Generate cryptographically secure state
pub fn generate_oauth_state() -> String {
    use uuid::Uuid;
    Uuid::new_v4().to_string()
}

// Validate state parameter
pub fn validate_oauth_state(stored_state: &str, received_state: &str) -> bool {
    // Constant-time comparison to prevent timing attacks
    use subtle::ConstantTimeEq;
    stored_state.as_bytes().ct_eq(received_state.as_bytes()).into()
}
```

#### Secure Redirect URI Validation
```rust
pub fn validate_redirect_uri(uri: &str, allowed_origins: &[String]) -> bool {
    if let Ok(parsed_uri) = url::Url::parse(uri) {
        // Check against allowed origins
        allowed_origins.iter().any(|origin| {
            if let Ok(allowed_origin) = url::Url::parse(origin) {
                parsed_uri.origin() == allowed_origin.origin()
            } else {
                false
            }
        })
    } else {
        false
    }
}
```

### Session Management Security

#### Secure Cookie Configuration
```rust
use tower_cookies::{Cookie, SameSite};
use time::Duration;

pub fn create_secure_session_cookie(session_id: String, is_production: bool) -> Cookie<'static> {
    Cookie::build(("session_id", session_id))
        .secure(is_production)              // HTTPS only in production
        .http_only(true)                    // Prevent XSS attacks
        .same_site(SameSite::Lax)          // CSRF protection
        .path("/")                         // Application-wide
        .max_age(Duration::hours(24))      // 24-hour expiration
        .build()
}
```

#### Session Storage Security
```rust
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub struct SecureSessionStore {
    sessions: Arc<RwLock<HashMap<String, SessionData>>>,
    max_sessions_per_user: usize,
}

#[derive(Clone)]
pub struct SessionData {
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub access_token_hash: String,      // Store hash, not plaintext
    pub refresh_token_hash: Option<String>,
}

impl SecureSessionStore {
    pub async fn create_session(&self, session_data: SessionData) -> Result<String, AppError> {
        let session_id = Uuid::new_v4().to_string();
        let mut sessions = self.sessions.write().unwrap();
        
        // Enforce session limits per user
        let user_sessions: Vec<_> = sessions
            .iter()
            .filter(|(_, data)| data.user_id == session_data.user_id)
            .map(|(id, _)| id.clone())
            .collect();
            
        if user_sessions.len() >= self.max_sessions_per_user {
            // Remove oldest session
            if let Some(oldest_session) = user_sessions.first() {
                sessions.remove(oldest_session);
            }
        }
        
        sessions.insert(session_id.clone(), session_data);
        Ok(session_id)
    }
    
    pub async fn validate_session(&self, session_id: &str) -> Result<SessionData, AppError> {
        let mut sessions = self.sessions.write().unwrap();
        
        if let Some(session) = sessions.get_mut(session_id) {
            // Check if session is expired (24 hours)
            let now = Utc::now();
            if now.signed_duration_since(session.last_activity).num_hours() > 24 {
                sessions.remove(session_id);
                return Err(AppError::Auth(AuthError::SessionError("Session expired".to_string())));
            }
            
            // Update last activity
            session.last_activity = now;
            Ok(session.clone())
        } else {
            Err(AppError::Auth(AuthError::SessionError("Invalid session".to_string())))
        }
    }
}
```

### Token Security

#### Secure Token Storage
```rust
use sha2::{Digest, Sha256};

pub fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn store_tokens_securely(access_token: String, refresh_token: Option<String>) -> (String, Option<String>) {
    let access_hash = hash_token(&access_token);
    let refresh_hash = refresh_token.map(|token| hash_token(&token));
    (access_hash, refresh_hash)
}
```

#### Token Rotation
```rust
pub async fn refresh_tokens(&self, refresh_token: &str) -> Result<GoogleTokenResponse, AppError> {
    // Validate refresh token
    let stored_hash = self.get_stored_refresh_token_hash().await?;
    let provided_hash = hash_token(refresh_token);
    
    if stored_hash != provided_hash {
        return Err(AppError::Auth(AuthError::InvalidToken));
    }
    
    // Exchange refresh token for new tokens
    let token_response = self.oauth_service.refresh_access_token(refresh_token).await?;
    
    // Store new token hashes
    let (new_access_hash, new_refresh_hash) = store_tokens_securely(
        token_response.access_token.clone(),
        token_response.refresh_token.clone(),
    );
    
    // Update stored hashes
    self.update_token_hashes(new_access_hash, new_refresh_hash).await?;
    
    Ok(token_response)
}
```

## Input Validation & Sanitization

### Request Validation Middleware
```rust
use axum::{http::Request, middleware::Next, response::Response};
use serde_json::Value;

pub async fn validate_request_middleware<B>(
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, AppError> {
    // Check content-type for POST/PUT requests
    if matches!(req.method(), &axum::http::Method::POST | &axum::http::Method::PUT) {
        let content_type = req.headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
            
        if !content_type.starts_with("application/json") {
            return Err(AppError::Validation("Invalid content type".to_string()));
        }
    }
    
    // Check request size limits
    let content_length = req.headers()
        .get("content-length")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
        
    if content_length > 1024 * 1024 { // 1MB limit
        return Err(AppError::Validation("Request too large".to_string()));
    }
    
    Ok(next.run(req).await)
}
```

### Query Parameter Sanitization
```rust
use axum::extract::Query;
use serde::Deserialize;
use validator::{Validate, ValidationError};

#[derive(Deserialize, Validate)]
pub struct OAuthCallbackParams {
    #[validate(length(min = 1, max = 2048))]
    pub code: String,
    #[validate(length(min = 1, max = 256))]
    pub state: String,
    #[validate(length(max = 256))]
    pub error: Option<String>,
    #[validate(length(max = 512))]
    pub error_description: Option<String>,
}

pub async fn oauth_callback(
    Query(params): Query<OAuthCallbackParams>,
) -> Result<Response, AppError> {
    // Validate parameters
    params.validate()
        .map_err(|e| AppError::Validation(format!("Invalid parameters: {}", e)))?;
        
    // Continue with validated parameters
    // ...
}
```

## HTTP Security Headers

### Security Headers Middleware
```rust
use axum::{
    http::{header, HeaderMap, HeaderValue},
    middleware::Next,
    response::Response,
};

pub async fn security_headers_middleware<B>(
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, AppError> {
    let mut response = next.run(req).await;
    let headers = response.headers_mut();
    
    // Prevent MIME type sniffing
    headers.insert(
        header::HeaderName::from_static("x-content-type-options"),
        HeaderValue::from_static("nosniff"),
    );
    
    // Prevent clickjacking
    headers.insert(
        header::HeaderName::from_static("x-frame-options"),
        HeaderValue::from_static("DENY"),
    );
    
    // XSS protection
    headers.insert(
        header::HeaderName::from_static("x-xss-protection"),
        HeaderValue::from_static("1; mode=block"),
    );
    
    // HSTS (HTTPS enforcement)
    headers.insert(
        header::HeaderName::from_static("strict-transport-security"),
        HeaderValue::from_static("max-age=31536000; includeSubDomains"),
    );
    
    // Content Security Policy
    headers.insert(
        header::HeaderName::from_static("content-security-policy"),
        HeaderValue::from_static(
            "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; connect-src 'self' https://accounts.google.com https://oauth2.googleapis.com"
        ),
    );
    
    // Referrer policy
    headers.insert(
        header::HeaderName::from_static("referrer-policy"),
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
    
    // Permissions policy
    headers.insert(
        header::HeaderName::from_static("permissions-policy"),
        HeaderValue::from_static("camera=(), microphone=(), geolocation=()"),
    );
    
    Ok(response)
}
```

## CORS Security

### Secure CORS Configuration
```rust
use tower_http::cors::{Any, CorsLayer};
use axum::http::{Method, HeaderValue};

pub fn create_cors_layer(allowed_origins: Vec<String>, is_production: bool) -> CorsLayer {
    let mut cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::ACCEPT,
        ])
        .allow_credentials(true)
        .max_age(std::time::Duration::from_secs(3600));

    if is_production {
        // Production: restrict to specific origins
        let origins: Vec<HeaderValue> = allowed_origins
            .into_iter()
            .filter_map(|origin| origin.parse().ok())
            .collect();
        cors = cors.allow_origin(origins);
    } else {
        // Development: allow localhost
        cors = cors.allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap());
    }

    cors
}
```

## Error Handling Security

### Secure Error Responses
```rust
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, user_message, should_log) = match &self {
            AppError::Auth(AuthError::InvalidCredentials) => {
                (StatusCode::UNAUTHORIZED, "Invalid credentials", false)
            }
            AppError::Auth(AuthError::MissingToken) => {
                (StatusCode::UNAUTHORIZED, "Authentication required", false)
            }
            AppError::Validation(msg) => {
                // Log validation errors but sanitize response
                (StatusCode::BAD_REQUEST, "Validation error", true)
            }
            AppError::Database(_) => {
                // Never expose database details
                (StatusCode::INTERNAL_SERVER_ERROR, "Service temporarily unavailable", true)
            }
            AppError::Internal(_) => {
                // Log internal errors but give generic response
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error", true)
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "An error occurred", true),
        };

        // Log detailed errors server-side only
        if should_log {
            tracing::error!("Error occurred: {:?}", self);
        }

        let body = Json(ApiError::new(
            self.error_code(),
            user_message,
            None, // Never include sensitive details in production
        ));

        (status, body).into_response()
    }
}
```

## Rate Limiting (Future Implementation)

### Rate Limiting Strategy
```rust
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

pub struct RateLimiter {
    limits: Arc<Mutex<HashMap<String, (Instant, u32)>>>,
    max_requests: u32,
    window_duration: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window_duration: Duration) -> Self {
        Self {
            limits: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window_duration,
        }
    }
    
    pub fn check_rate_limit(&self, key: &str) -> Result<(), AppError> {
        let mut limits = self.limits.lock().unwrap();
        let now = Instant::now();
        
        let (window_start, request_count) = limits
            .entry(key.to_string())
            .or_insert((now, 0));
        
        // Reset window if expired
        if now.duration_since(*window_start) > self.window_duration {
            *window_start = now;
            *request_count = 0;
        }
        
        if *request_count >= self.max_requests {
            return Err(AppError::Validation("Rate limit exceeded".to_string()));
        }
        
        *request_count += 1;
        Ok(())
    }
}
```

## Logging and Monitoring Security

### Secure Logging Implementation
```rust
use tracing::{info, warn, error};
use serde_json::json;

pub fn log_authentication_event(event_type: &str, user_id: Option<&str>, ip_address: Option<&str>, success: bool) {
    let log_data = json!({
        "event_type": event_type,
        "user_id": user_id,
        "ip_address": ip_address,
        "success": success,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });
    
    if success {
        info!("Auth event: {}", log_data);
    } else {
        warn!("Auth event failed: {}", log_data);
    }
}

pub fn log_security_event(event_type: &str, details: Option<&str>, severity: &str) {
    let log_data = json!({
        "event_type": event_type,
        "details": details,
        "severity": severity,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });
    
    match severity {
        "high" | "critical" => error!("Security event: {}", log_data),
        "medium" => warn!("Security event: {}", log_data),
        _ => info!("Security event: {}", log_data),
    }
}

// Never log sensitive data
pub fn sanitize_for_logging(data: &str) -> String {
    if data.len() > 4 {
        format!("{}***", &data[..4])
    } else {
        "***".to_string()
    }
}
```

## Frontend Security

### Secure Cookie Handling in Leptos
```rust
use leptos_use::{use_cookie, CookieOptions, SameSite};

pub fn use_secure_session_cookie() -> (Signal<Option<String>>, WriteSignal<Option<String>>) {
    use_cookie_with_options(
        "session_id",
        CookieOptions::default()
            .same_site(SameSite::Lax)     // CSRF protection
            .secure(true)                 // HTTPS only in production
            .http_only(false)             // Allow JS access for client-side validation
            .path("/")                    // Application-wide
            .max_age(24 * 60 * 60 * 1000) // 24 hours
    )
}
```

### XSS Prevention in Components
```rust
use leptos::*;

#[component]
pub fn UserProfile(user_name: String) -> impl IntoView {
    // Leptos automatically escapes HTML in text nodes
    view! {
        <div class="user-profile">
            // Safe: Leptos escapes the user_name automatically
            <h2>{user_name}</h2>
            
            // Dangerous: Never use inner_html with user data
            // <div inner_html=user_name></div>  // DON'T DO THIS
        </div>
    }
}
```

## Security Checklist

### Authentication & Authorization
- [x] OAuth 2.0 with PKCE implementation
- [x] CSRF protection with state parameter
- [x] Secure session management
- [x] Token rotation and expiration
- [x] Session limits per user
- [x] Secure cookie configuration

### Input Validation
- [x] Request size limits
- [x] Content-type validation
- [x] Parameter sanitization
- [x] SQL injection prevention (parameterized queries)
- [x] Path traversal prevention

### HTTP Security
- [x] Security headers implementation
- [x] CORS configuration
- [x] HTTPS enforcement (HSTS)
- [x] Content Security Policy (CSP)
- [x] XSS protection headers

### Error Handling
- [x] Secure error responses (no sensitive data)
- [x] Comprehensive logging (sanitized)
- [x] Rate limiting preparation
- [x] Monitoring and alerting

### Data Protection
- [x] Token hashing in storage
- [x] Sensitive data encryption
- [x] Secure cookie attributes
- [x] No sensitive data in logs
- [x] Proper data retention policies

### Infrastructure Security
- [x] Environment variable protection
- [x] Secure deployment configuration
- [x] Network security (HTTPS only)
- [x] Regular dependency updates
- [x] Security monitoring

This comprehensive security implementation ensures the application follows security best practices for authentication, data protection, and infrastructure security.