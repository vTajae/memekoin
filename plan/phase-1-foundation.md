# Phase 1: Foundation

**Duration**: 1-2 days  
**Priority**: Critical  
**Status**: Ready to implement

## Overview

Fix critical compilation issues and establish core infrastructure for the Leptos fullstack Cloudflare worker application.

## Critical Issues to Fix

### 1. Cargo.toml Dependency Conflict
**Issue**: Duplicate `tower-service` dependency at line 52
```toml
# Remove this duplicate line:
tower-service = "0.3.3"  # Line 52 - REMOVE
```

**Solution**: Remove the duplicate entry, keep only line 21

### 2. Missing Dependencies
Add required dependencies for JSON handling and OAuth:

```toml
[dependencies]
# Add these to Cargo.toml
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.0", features = ["v4", "js"] }
base64 = "0.21"
url = "2.4"
reqwest = { version = "0.11", default-features = false, features = ["json", "rustls-tls"] }
```

## Module Implementation Tasks

### 1. Complete Handler Module Structure
Create missing handler files:

```
src/handler/
├── mod.rs          # Handler module exports
├── auth.rs         # Authentication handlers
├── health.rs       # Health check handlers
└── user.rs         # User management handlers
```

### 2. Implement Service Layer
Create business logic services:

```
src/service/
├── mod.rs          # Service module exports
├── auth.rs         # Authentication service (partially exists)
├── user.rs         # User management service
└── oauth.rs        # OAuth integration service
```

### 3. Repository Pattern Implementation
Create data access layer:

```
src/repository/
├── mod.rs          # Repository module exports
├── user.rs         # User data repository
└── session.rs      # Session storage repository
```

### 4. Entity Definitions
Define data structures:

```
src/entity/
├── mod.rs          # Entity module exports
├── user.rs         # User entity
├── session.rs      # Session entity
└── oauth.rs        # OAuth token entity
```

### 5. DTO Definitions
API request/response structures:

```
src/dto/
├── mod.rs          # DTO module exports
├── auth.rs         # Authentication DTOs
├── user.rs         # User DTOs
└── response.rs     # Standard API response formats
```

## Implementation Checklist

### Critical Fixes
- [ ] Remove duplicate `tower-service` dependency from Cargo.toml:52
- [ ] Add required dependencies for serde, uuid, base64, url, reqwest
- [ ] Verify compilation with `cargo check`

### Module Structure
- [ ] Create `src/handler/mod.rs` with proper exports
- [ ] Create `src/service/mod.rs` with proper exports
- [ ] Create `src/repository/mod.rs` with proper exports
- [ ] Create `src/entity/mod.rs` with proper exports
- [ ] Create `src/dto/mod.rs` with proper exports

### Basic Implementations
- [ ] Implement health check handler in `src/handler/health.rs`
- [ ] Create basic user entity in `src/entity/user.rs`
- [ ] Create standard API response DTOs in `src/dto/response.rs`
- [ ] Update `src/lib.rs` imports to include new modules

## Validation Steps

### Compilation Test
```bash
cd axum-worker
cargo check --message-format=json
```
**Expected**: No compilation errors

### Module Loading Test
```bash
cargo build --target wasm32-unknown-unknown
```
**Expected**: Successful WASM build

### Run Application
```bash
cd axum-worker
wrangler dev
curl http://127.0.0.1:8787/api/health
```
**Expected**: Successful build 

### Basic Health Check
After implementation, test:
```bash
curl https://your-worker.workers.dev/api/health
```
**Expected**: JSON health response

## Success Criteria

✅ **Clean Compilation** - `cargo check` passes without errors  
✅ **WASM Build** - Successfully builds for `wasm32-unknown-unknown` target  
✅ **Module Structure** - All required modules exist and export properly  
✅ **Basic Health Endpoint** - Health check returns proper JSON response  
✅ **Error Handling** - Proper error responses for invalid endpoints

## Next Phase

Once Phase 1 is complete, proceed to [Phase 2: Authentication Implementation](./phase-2-authentication.md)

## File Templates

### Basic Handler Template
```rust
// src/handler/{name}.rs
use axum::{extract::State, Json};
use crate::{state::AppState, dto::response::ApiResponse, error::AppError};

pub async fn handler_function(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    Ok(Json(ApiResponse::success(None, "Success message")))
}
```

### Basic Service Template  
```rust
// src/service/{name}.rs
use crate::{error::AppError, entity::{/* relevant entities */}};

pub struct ServiceName {
    // Dependencies
}

impl ServiceName {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn service_method(&self) -> Result<(), AppError> {
        // Implementation
        Ok(())
    }
}
```