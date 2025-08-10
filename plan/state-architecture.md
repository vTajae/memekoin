# State, Layering, and DI in Axum Worker

This document captures the architecture decision for dependency management, layering, and environment configuration.

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
- Added `Database::from_url(&str)`; AppState resolves URL and constructs the DB client.
- AppState now uses `OnceLock<Arc<Repo>>` and exposes:
  - `fn user_repo(&self) -> Arc<UserRepository>`
  - `fn session_repo(&self) -> Arc<SessionRepository>`
- Optional helpers (not cached):
  - `fn make_session_service(&self) -> SessionService`
  - `fn make_oauth_service(&self) -> OAuthService`
  - `fn make_simplified_oauth_service(&self) -> SimplifiedOAuthService`
- Handlers/middleware call the repo getters or construct services on demand.

## Security
- Secrets preferred over plain env vars for DB URLs.
- Session cookie should be HttpOnly; SameSite=Lax (or Strict) and Secure in production.
- Avoid logging sensitive values; truncate when necessary.

## Usage Examples
- In a handler:
  - `let users = state.user_repo();`
  - `let sessions = state.session_repo();`
  - `let svc = state.make_session_service();`
- OAuth:
  - `let oauth = state.make_oauth_service();`
  - `let simple_oauth = state.make_simplified_oauth_service();`

## References
- Java (Controller → Service → Repository): https://github.com/vTajae/JavaFullStack
- Rust worker example: https://github.com/vTajae/rusty-worker/blob/main/src/handler/auth.rs

## Migration Notes
- Replace `state.user_repository()`/`state.session_repository()` Option getters with non-optional `state.user_repo()`/`state.session_repo()`.
- Construct services on demand; do not store services in AppState.
- Centralize DB URL resolution in AppState; pass to `Database::from_url`.
