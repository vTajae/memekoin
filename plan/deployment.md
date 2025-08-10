# Deployment Guide

## Overview

Comprehensive deployment guide for the Leptos fullstack Rust Cloudflare worker application with Google OAuth integration.

## Prerequisites

### Required Accounts & Tools
```yaml
accounts:
  - Cloudflare Account (Workers plan)
  - Google Cloud Console (OAuth credentials)
  - PostgreSQL Database (Neon, Supabase, or similar)

tools:
  - Rust toolchain (stable)
  - wrangler CLI (Cloudflare Workers CLI)
  - trunk (Leptos build tool)
  - git (version control)
```

### Installation Commands
```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install wrangler CLI
npm install -g wrangler

# Install trunk for Leptos builds
cargo install --locked trunk

# Verify installations
wrangler --version
trunk --version
cargo --version
```

## Environment Configuration

### Google OAuth Setup

#### 1. Create Google Cloud Project
```bash
# Go to Google Cloud Console
# https://console.cloud.google.com/

# Create new project or select existing
# Enable Google+ API and OAuth consent screen
```

#### 2. Configure OAuth Consent Screen
```yaml
oauth_consent:
  application_name: "Your App Name"
  authorized_domains: 
    - your-domain.com
    - your-worker.workers.dev
  scopes:
    - email
    - profile
    - openid
```

#### 3. Create OAuth Credentials
```yaml
credential_type: "Web application"
authorized_origins:
  - https://your-worker.workers.dev
  - https://your-domain.com  
authorized_redirect_uris:
  - https://your-worker.workers.dev/api/auth/oauth/callback
  - https://your-domain.com/api/auth/oauth/callback
```

### Database Setup

#### PostgreSQL Database (Recommended: Neon)
```bash
# Sign up at https://neon.tech/
# Create new database
# Copy connection string

# Example connection string format:
# postgresql://username:password@host/database?sslmode=require
```

#### Database Schema
```sql
-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    name VARCHAR(255),
    picture VARCHAR(512),
    verified_email BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Sessions table
CREATE TABLE sessions (
    session_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    access_token_hash VARCHAR(64) NOT NULL,
    refresh_token_hash VARCHAR(64),
    token_expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    last_activity TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    user_agent TEXT,
    ip_address INET
);

-- Indexes for performance
CREATE INDEX idx_sessions_user_id ON sessions(user_id);
CREATE INDEX idx_sessions_last_activity ON sessions(last_activity);
CREATE INDEX idx_users_email ON users(email);
```

## Cloudflare Workers Configuration

### wrangler.toml Configuration
```toml
# wrangler.toml - Backend configuration
name = "your-app-api"
main = "src/lib.rs"
compatibility_date = "2024-01-15"

[build]
command = "cargo build --release --target wasm32-unknown-unknown"

[[kv_namespaces]]
binding = "SESSIONS"
preview_id = "preview_session_store_id"
id = "production_session_store_id"

[vars]
ENVIRONMENT = "production"
BASE_URL = "https://your-worker.workers.dev"

[env.development]
name = "your-app-api-dev"
vars = { ENVIRONMENT = "development", BASE_URL = "http://localhost:8787" }

[env.staging]
name = "your-app-api-staging"
vars = { ENVIRONMENT = "staging", BASE_URL = "https://your-app-staging.workers.dev" }

[env.production]
name = "your-app-api"
vars = { ENVIRONMENT = "production", BASE_URL = "https://your-app.workers.dev" }
```

### Environment Variables Setup
```bash
# Set production secrets
wrangler secret put GOOGLE_CLIENT_ID
wrangler secret put GOOGLE_CLIENT_SECRET
wrangler secret put DATABASE_URL
wrangler secret put SESSION_SECRET

# Set staging secrets
wrangler secret put GOOGLE_CLIENT_ID --env staging
wrangler secret put GOOGLE_CLIENT_SECRET --env staging
wrangler secret put DATABASE_URL --env staging
wrangler secret put SESSION_SECRET --env staging
```

## Build Process

### Backend Build (Axum Worker)
```bash
cd axum-worker

# Development build
wrangler dev

# Production build and deploy
wrangler publish

# Deploy to specific environment
wrangler publish --env staging
wrangler publish --env production
```

### Frontend Build (Leptos)
```bash
cd leptos-frontend

# Development server
trunk serve

# Production build
trunk build --release

# Deploy to Cloudflare Pages (optional)
# Upload dist/ folder to Cloudflare Pages
```

### Combined Build Script
```bash
#!/bin/bash
# build-and-deploy.sh

set -e

echo "Building backend..."
cd axum-worker
cargo check
wrangler publish --env production

echo "Building frontend..."
cd ../leptos-frontend
trunk build --release

echo "Deployment complete!"
echo "Backend: https://your-app.workers.dev"
echo "Frontend: Upload dist/ to your hosting provider"
```

## Custom Domain Setup

### Cloudflare Workers Custom Domain
```bash
# Add custom domain through Cloudflare dashboard
# Workers & Pages > your-worker > Settings > Triggers
# Add custom domain: api.your-domain.com

# Update OAuth redirect URIs to include custom domain
# Update CORS origins in application
```

### SSL Certificate
```yaml
ssl_configuration:
  type: "Full (strict)"
  certificate: "Universal SSL" # Automatic with Cloudflare
  hsts: "Enabled"
  min_tls_version: "1.2"
```

## Monitoring & Logging

### Cloudflare Analytics
```yaml
available_metrics:
  - Request count
  - Error rate
  - Response time
  - Geographic distribution
  - Status code distribution
```

### Custom Logging Setup
```rust
// Enhanced logging configuration
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

pub fn init_logging() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().json())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();
}

// Log important events
#[tracing::instrument]
pub async fn handle_oauth_callback(/* params */) -> Result<Response, AppError> {
    tracing::info!("OAuth callback received");
    // Implementation...
}
```

### Error Monitoring Integration
```rust
// Optional: Sentry integration
use sentry_tracing::SentryLayer;

pub fn init_error_monitoring() {
    let _guard = sentry::init("YOUR_SENTRY_DSN");
    
    tracing_subscriber::registry()
        .with(SentryLayer::new())
        .with(tracing_subscriber::fmt::layer())
        .init();
}
```

## Health Checks & Monitoring

### Health Check Endpoints
```rust
// Comprehensive health check
#[tracing::instrument]
pub async fn detailed_health_check(State(state): State<AppState>) -> Result<Json<HealthResponse>, AppError> {
    let mut health_status = HealthStatus {
        status: "healthy".to_string(),
        timestamp: chrono::Utc::now(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        environment: state.config.environment.clone(),
        services: ServiceHealth::default(),
    };

    // Check database connection
    match check_database_health(&state).await {
        Ok(_) => health_status.services.database = "healthy".to_string(),
        Err(_) => {
            health_status.services.database = "unhealthy".to_string();
            health_status.status = "degraded".to_string();
        }
    }

    // Check OAuth configuration
    health_status.services.oauth = if validate_oauth_config(&state.google_oauth_config) {
        "healthy".to_string()
    } else {
        health_status.status = "degraded".to_string();
        "unhealthy".to_string()
    };

    Ok(Json(ApiResponse::success(Some(health_status), None)))
}
```

### External Monitoring Setup
```yaml
monitoring_urls:
  - https://your-app.workers.dev/api/health
  - https://your-app.workers.dev/api/health/detailed

recommended_tools:
  - UptimeRobot (free tier available)
  - Pingdom
  - Better Stack (former Better Uptime)
  - Cloudflare Analytics
```

## Performance Optimization

### Bundle Size Optimization
```toml
# Cargo.toml optimizations
[profile.release]
opt-level = "s"          # Optimize for size
lto = true              # Link-time optimization
codegen-units = 1       # Single codegen unit
panic = "abort"         # Smaller panic handler

[package.metadata.wasm-pack.profile.release]
wasm-opt = ["-Os", "--enable-mutable-globals"]
```

### Cloudflare Workers Optimizations
```rust
// Minimize cold start time
use worker::*;

#[event(fetch)]
async fn fetch(req: HttpRequest, env: Env, _ctx: Context) -> Result<Response> {
    // Initialize logging once
    static INIT: std::sync::Once = std::sync::Once::new();
    INIT.call_once(|| {
        console_error_panic_hook::set_once();
        init_logging();
    });

    // Use shared state to avoid reinitializing
    static STATE: std::sync::OnceLock<AppState> = std::sync::OnceLock::new();
    let app_state = STATE.get_or_init(|| AppState::new(env).unwrap());

    // Handle request
    handle_request(req, app_state).await
}
```

## Security Hardening

### Production Security Configuration
```rust
// Security middleware stack
pub fn create_production_middleware() -> ServiceBuilder<Stack<AuthMiddleware, Stack<SecurityHeadersMiddleware, CorsLayer>>> {
    ServiceBuilder::new()
        .layer(AuthMiddleware::new())
        .layer(SecurityHeadersMiddleware::new())
        .layer(create_production_cors())
}

pub fn create_production_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin("https://your-domain.com".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
        .allow_credentials(true)
        .max_age(Duration::from_secs(3600))
}
```

### Environment-Specific Configuration
```rust
impl AppState {
    pub async fn new(env: Env) -> Result<Self, AppError> {
        let environment = env.var("ENVIRONMENT")?.to_string();
        
        let google_oauth_config = GoogleOAuthConfig {
            client_id: env.secret("GOOGLE_CLIENT_ID")?.to_string(),
            client_secret: env.secret("GOOGLE_CLIENT_SECRET")?.to_string(),
            redirect_uri: format!("{}/api/auth/oauth/callback", env.var("BASE_URL")?),
            // Production vs development scopes
            scopes: if environment == "production" {
                vec!["openid".to_string(), "email".to_string(), "profile".to_string()]
            } else {
                vec!["openid".to_string(), "email".to_string()]
            },
        };

        Ok(Self {
            google_oauth_config,
            environment,
            // Initialize services...
        })
    }
}
```

## CI/CD Pipeline

### GitHub Actions Workflow
```yaml
# .github/workflows/deploy.yml
name: Deploy to Cloudflare Workers

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

env:
  RUST_VERSION: 1.75.0

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: ${{ env.RUST_VERSION }}
          target: wasm32-unknown-unknown
          override: true
          components: rustfmt, clippy
      
      - name: Cache cargo registry
        uses: actions/cache@v3
        with:
          path: ~/.cargo/registry
          key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Run tests
        run: |
          cd axum-worker
          cargo test
      
      - name: Run clippy
        run: |
          cd axum-worker
          cargo clippy -- -D warnings
      
      - name: Check formatting
        run: |
          cd axum-worker
          cargo fmt --all -- --check

  deploy-staging:
    if: github.event_name == 'pull_request'
    needs: test
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install wrangler
        run: npm install -g wrangler
      
      - name: Deploy to staging
        env:
          CLOUDFLARE_API_TOKEN: ${{ secrets.CLOUDFLARE_API_TOKEN }}
        run: |
          cd axum-worker
          wrangler publish --env staging

  deploy-production:
    if: github.ref == 'refs/heads/main'
    needs: test
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install wrangler
        run: npm install -g wrangler
      
      - name: Deploy to production
        env:
          CLOUDFLARE_API_TOKEN: ${{ secrets.CLOUDFLARE_API_TOKEN }}
        run: |
          cd axum-worker
          wrangler publish --env production
```

### Manual Deployment Script
```bash
#!/bin/bash
# deploy.sh - Manual deployment script

set -e

ENVIRONMENT=${1:-production}

echo "Deploying to environment: $ENVIRONMENT"

# Validate environment
if [[ ! "$ENVIRONMENT" =~ ^(development|staging|production)$ ]]; then
    echo "Error: Invalid environment. Use: development, staging, or production"
    exit 1
fi

# Run tests first
echo "Running tests..."
cd axum-worker
cargo test
cargo clippy -- -D warnings
cargo fmt --all -- --check

# Deploy
echo "Deploying to Cloudflare Workers..."
if [ "$ENVIRONMENT" = "production" ]; then
    wrangler publish --env production
else
    wrangler publish --env "$ENVIRONMENT"
fi

echo "Deployment completed successfully!"
echo "URL: https://your-app-$ENVIRONMENT.workers.dev"
```

## Rollback Strategy

### Quick Rollback
```bash
# Rollback to previous version
wrangler rollback --env production

# Deploy specific version
git checkout <previous-commit>
wrangler publish --env production
git checkout main
```

### Blue-Green Deployment
```bash
# Deploy to staging first
wrangler publish --env staging

# Test staging environment
curl https://your-app-staging.workers.dev/api/health

# If tests pass, promote to production
wrangler publish --env production
```

## Troubleshooting

### Common Issues

#### Build Errors
```bash
# WASM target not installed
rustup target add wasm32-unknown-unknown

# Dependency version conflicts
cargo clean
cargo update
```

#### Runtime Errors
```bash
# Check worker logs
wrangler tail

# Check worker logs for specific environment
wrangler tail --env production

# Debug locally
wrangler dev --local
```

#### Database Connection Issues
```bash
# Test database connection
psql $DATABASE_URL -c "SELECT version();"

# Check connection limits
# Most managed databases have connection limits
```

### Debug Configuration
```toml
# wrangler.toml debug settings
[env.debug]
name = "your-app-debug"
vars = { 
  ENVIRONMENT = "debug", 
  RUST_LOG = "debug",
  BASE_URL = "http://localhost:8787" 
}
```

## Deployment Checklist

### Pre-Deployment
- [ ] All tests passing
- [ ] Code review completed
- [ ] Environment variables configured
- [ ] Database schema up to date
- [ ] OAuth credentials configured
- [ ] Health checks working

### Deployment
- [ ] Deploy to staging first
- [ ] Run smoke tests on staging
- [ ] Check health endpoints
- [ ] Verify authentication flow
- [ ] Test critical user paths

### Post-Deployment
- [ ] Monitor error rates
- [ ] Check performance metrics
- [ ] Verify database connections
- [ ] Test OAuth flow end-to-end
- [ ] Check security headers
- [ ] Monitor for any alerts

### Rollback Plan
- [ ] Previous version tagged in git
- [ ] Rollback script tested
- [ ] Database migration rollback plan
- [ ] Communication plan for users

This comprehensive deployment guide ensures a smooth and secure deployment process for the Leptos fullstack application on Cloudflare Workers.