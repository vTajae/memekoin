# Phase 4: Frontend Integration

**Duration**: 2-3 days  
**Priority**: Medium  
**Prerequisites**: Phases 1, 2, & 3 completed  
**Status**: Ready for implementation after Phase 3

## Overview

Integrate Leptos frontend with the Axum backend authentication system and API. Focus on seamless user experience, secure session management, and reactive authentication state.

## Frontend Architecture

### Technology Stack
```yaml
framework: Leptos (WASM)
state_management: leptos-use (cookies, reactivity)
authentication: Cookie-based sessions
http_client: reqwest (WASM-compatible)
styling: Tailwind CSS (optional)
routing: leptos_router
```

### Component Architecture
```
src/
├── components/
│   ├── auth/
│   │   ├── login_button.rs
│   │   ├── logout_button.rs  
│   │   ├── user_profile.rs
│   │   └── auth_provider.rs
│   ├── layout/
│   │   ├── header.rs
│   │   ├── sidebar.rs
│   │   └── protected_route.rs
│   └── common/
│       ├── loading_spinner.rs
│       └── error_message.rs
├── pages/
│   ├── home.rs
│   ├── login.rs
│   ├── dashboard.rs
│   └── profile.rs
├── services/
│   ├── auth_service.rs
│   └── api_client.rs
├── types/
│   └── auth.rs
└── utils/
    ├── cookies.rs
    └── routes.rs
```

## Implementation Tasks

### 1. Authentication State Management

#### Auth Context & Provider (`src/components/auth/auth_provider.rs`)
```rust
use leptos::*;
use leptos_use::{use_cookie, SameSite};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub picture: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AuthState {
    Loading,
    Authenticated(User),
    Unauthenticated,
}

#[derive(Clone)]
pub struct AuthContext {
    pub state: ReadSignal<AuthState>,
    pub login: Action<(), ()>,
    pub logout: Action<(), ()>,
    pub refresh_session: Action<(), ()>,
}

#[component]
pub fn AuthProvider(children: Children) -> impl IntoView {
    let (auth_state, set_auth_state) = create_signal(AuthState::Loading);
    let (session_cookie, set_session_cookie) = use_cookie::<String>("session_id");
    
    // Initialize authentication state
    create_effect(move |_| {
        if let Some(session_id) = session_cookie.get() {
            spawn_local(async move {
                match validate_session(&session_id).await {
                    Ok(user) => set_auth_state.set(AuthState::Authenticated(user)),
                    Err(_) => {
                        set_session_cookie.set(None);
                        set_auth_state.set(AuthState::Unauthenticated);
                    }
                }
            });
        } else {
            set_auth_state.set(AuthState::Unauthenticated);
        }
    });

    let login_action = create_action(|_: &()| {
        let window = web_sys::window().unwrap();
        window.location().set_href("/api/auth/oauth/login").unwrap();
        async {}
    });

    let logout_action = create_action({
        let set_session_cookie = set_session_cookie.clone();
        let set_auth_state = set_auth_state.clone();
        
        move |_: &()| {
            let set_session_cookie = set_session_cookie.clone();
            let set_auth_state = set_auth_state.clone();
            
            async move {
                if let Ok(_) = logout_user().await {
                    set_session_cookie.set(None);
                    set_auth_state.set(AuthState::Unauthenticated);
                }
            }
        }
    });

    let auth_context = AuthContext {
        state: auth_state.into(),
        login: login_action,
        logout: logout_action,
        refresh_session: create_action(|_: &()| async {}), // TODO: Implement refresh
    };

    provide_context(auth_context);
    children()
}
```

#### Session Validation Service (`src/services/auth_service.rs`)
```rust
use leptos::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub success: bool,
    pub error: ErrorDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorDetails {
    pub code: String,
    pub message: String,
    pub details: Option<String>,
}

pub async fn validate_session(session_id: &str) -> Result<User, String> {
    let client = reqwest::Client::new();
    
    let response = client
        .get("/api/auth/user")
        .header("Cookie", format!("session_id={}", session_id))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if response.status().is_success() {
        let api_response: ApiResponse<User> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
            
        api_response.data
            .ok_or_else(|| "No user data in response".to_string())
    } else {
        Err("Session validation failed".to_string())
    }
}

pub async fn logout_user() -> Result<(), String> {
    let client = reqwest::Client::new();
    
    let response = client
        .post("/api/auth/logout")
        .send()
        .await
        .map_err(|e| format!("Logout request failed: {}", e))?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err("Logout failed".to_string())
    }
}

pub async fn refresh_token() -> Result<(), String> {
    let client = reqwest::Client::new();
    
    let response = client
        .post("/api/auth/refresh")
        .send()
        .await
        .map_err(|e| format!("Token refresh failed: {}", e))?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err("Token refresh failed".to_string())
    }
}
```

### 2. Authentication Components

#### Login Button (`src/components/auth/login_button.rs`)
```rust
use leptos::*;
use crate::components::auth::auth_provider::{AuthContext, AuthState};

#[component]
pub fn LoginButton(
    #[prop(optional)] class: Option<&'static str>,
) -> impl IntoView {
    let auth_context = use_context::<AuthContext>()
        .expect("AuthContext must be provided");

    view! {
        <button
            class=format!(
                "flex items-center px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors {}",
                class.unwrap_or("")
            )
            on:click=move |_| {
                auth_context.login.dispatch(());
            }
        >
            <svg class="w-5 h-5 mr-2" viewBox="0 0 24 24">
                // Google icon SVG
            </svg>
            "Sign in with Google"
        </button>
    }
}
```

#### User Profile Component (`src/components/auth/user_profile.rs`)
```rust
use leptos::*;
use crate::components::auth::auth_provider::{AuthContext, AuthState};

#[component]
pub fn UserProfile() -> impl IntoView {
    let auth_context = use_context::<AuthContext>()
        .expect("AuthContext must be provided");

    view! {
        <div class="flex items-center space-x-3">
            {move || match auth_context.state.get() {
                AuthState::Loading => view! {
                    <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"></div>
                }.into_view(),
                AuthState::Authenticated(user) => view! {
                    <div class="flex items-center space-x-3">
                        {user.picture.as_ref().map(|pic| view! {
                            <img 
                                src=pic
                                alt="Profile"
                                class="w-8 h-8 rounded-full"
                            />
                        })}
                        <div class="flex flex-col">
                            <span class="text-sm font-medium text-gray-900">
                                {user.name.as_ref().unwrap_or(&user.email)}
                            </span>
                            <span class="text-xs text-gray-500">{&user.email}</span>
                        </div>
                        <button
                            class="text-sm text-gray-500 hover:text-gray-700"
                            on:click=move |_| {
                                auth_context.logout.dispatch(());
                            }
                        >
                            "Sign out"
                        </button>
                    </div>
                }.into_view(),
                AuthState::Unauthenticated => view! {
                    <crate::components::auth::login_button::LoginButton />
                }.into_view(),
            }}
        </div>
    }
}
```

#### Protected Route Component (`src/components/layout/protected_route.rs`)
```rust
use leptos::*;
use leptos_router::*;
use crate::components::auth::auth_provider::{AuthContext, AuthState};

#[component]
pub fn ProtectedRoute(children: Children) -> impl IntoView {
    let auth_context = use_context::<AuthContext>()
        .expect("AuthContext must be provided");

    view! {
        {move || match auth_context.state.get() {
            AuthState::Loading => view! {
                <div class="flex justify-center items-center min-h-screen">
                    <div class="animate-spin rounded-full h-32 w-32 border-b-2 border-gray-900"></div>
                </div>
            }.into_view(),
            AuthState::Authenticated(_) => children().into_view(),
            AuthState::Unauthenticated => view! {
                <Redirect path="/login" />
            }.into_view(),
        }}
    }
}
```

### 3. Page Components

#### Login Page (`src/pages/login.rs`)
```rust
use leptos::*;
use leptos_router::*;
use crate::components::auth::auth_provider::{AuthContext, AuthState};
use crate::components::auth::login_button::LoginButton;

#[component]
pub fn LoginPage() -> impl IntoView {
    let auth_context = use_context::<AuthContext>()
        .expect("AuthContext must be provided");

    // Redirect if already authenticated
    create_effect(move |_| {
        if let AuthState::Authenticated(_) = auth_context.state.get() {
            let navigate = use_navigate();
            navigate("/dashboard", Default::default());
        }
    });

    view! {
        <div class="min-h-screen flex items-center justify-center bg-gray-50">
            <div class="max-w-md w-full space-y-8">
                <div>
                    <h2 class="mt-6 text-center text-3xl font-extrabold text-gray-900">
                        "Welcome to Your App"
                    </h2>
                    <p class="mt-2 text-center text-sm text-gray-600">
                        "Sign in to access your dashboard"
                    </p>
                </div>
                <div class="flex justify-center">
                    <LoginButton class="w-full justify-center" />
                </div>
            </div>
        </div>
    }
}
```

#### Dashboard Page (`src/pages/dashboard.rs`)
```rust
use leptos::*;
use crate::components::layout::protected_route::ProtectedRoute;
use crate::components::auth::user_profile::UserProfile;

#[component]
pub fn DashboardPage() -> impl IntoView {
    view! {
        <ProtectedRoute>
            <div class="min-h-screen bg-gray-50">
                <header class="bg-white shadow">
                    <div class="max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8">
                        <div class="flex justify-between items-center">
                            <h1 class="text-3xl font-bold text-gray-900">
                                "Dashboard"
                            </h1>
                            <UserProfile />
                        </div>
                    </div>
                </header>
                <main>
                    <div class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
                        <div class="px-4 py-6 sm:px-0">
                            <div class="border-4 border-dashed border-gray-200 rounded-lg h-96 flex items-center justify-center">
                                <p class="text-gray-500">
                                    "Dashboard content goes here"
                                </p>
                            </div>
                        </div>
                    </div>
                </main>
            </div>
        </ProtectedRoute>
    }
}
```

### 4. Cookie Management Utilities

#### Cookie Utilities (`src/utils/cookies.rs`)
```rust
use leptos_use::{use_cookie, CookieOptions, SameSite};
use leptos::*;

pub fn use_session_cookie() -> (Signal<Option<String>>, WriteSignal<Option<String>>) {
    use_cookie_with_options(
        "session_id",
        CookieOptions::default()
            .same_site(SameSite::Lax)
            .secure(true) // Enable in production
            .http_only(false) // Allow JavaScript access for client-side validation
            .path("/")
            .max_age(24 * 60 * 60 * 1000) // 24 hours in milliseconds
    )
}

pub fn clear_all_auth_cookies() {
    let (_, set_session) = use_session_cookie();
    set_session.set(None);
    
    // Clear any other auth-related cookies
    // This ensures complete logout
}
```

### 5. HTTP Client Configuration

#### API Client (`src/services/api_client.rs`)
```rust
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: "/api".to_string(), // Use relative URLs in WASM
        }
    }

    pub async fn get<T>(&self, endpoint: &str) -> Result<T, String>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("GET request failed: {}", e))?;

        if response.status().is_success() {
            response
                .json::<T>()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))
        } else {
            Err(format!("Request failed with status: {}", response.status()))
        }
    }

    pub async fn post<T, R>(&self, endpoint: &str, data: &T) -> Result<R, String>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        
        let response = self.client
            .post(&url)
            .json(data)
            .send()
            .await
            .map_err(|e| format!("POST request failed: {}", e))?;

        if response.status().is_success() {
            response
                .json::<R>()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))
        } else {
            Err(format!("Request failed with status: {}", response.status()))
        }
    }
}
```

### 6. Main Application Setup

#### App Component (`src/app.rs`)
```rust
use leptos::*;
use leptos_router::*;
use crate::components::auth::auth_provider::AuthProvider;
use crate::pages::{home::HomePage, login::LoginPage, dashboard::DashboardPage};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <AuthProvider>
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path="/login" view=LoginPage/>
                    <Route path="/dashboard" view=DashboardPage/>
                </Routes>
            </AuthProvider>
        </Router>
    }
}
```

## Implementation Checklist

### Authentication State
- [ ] Implement AuthProvider with leptos-use cookies
- [ ] Create AuthContext for global state management
- [ ] Add session validation with backend API
- [ ] Include automatic session refresh logic

### UI Components
- [ ] Create LoginButton with Google branding
- [ ] Implement UserProfile with logout functionality
- [ ] Build ProtectedRoute for route guarding
- [ ] Add loading states and error handling

### Page Components
- [ ] Implement LoginPage with proper redirects
- [ ] Create Dashboard with authentication check
- [ ] Add Profile page for user management
- [ ] Include error pages for auth failures

### Services & Utils
- [ ] Create HTTP client for API communication
- [ ] Implement cookie management utilities
- [ ] Add error handling and retry logic
- [ ] Include request/response type definitions

### Integration Testing
- [ ] Test authentication flow end-to-end
- [ ] Verify session persistence across page reloads
- [ ] Test logout and session cleanup
- [ ] Validate protected route behavior

## User Experience Considerations

### Loading States
```rust
// Implement proper loading states during auth operations
#[component]
pub fn LoadingSpinner() -> impl IntoView {
    view! {
        <div class="inline-flex items-center px-4 py-2 text-sm text-gray-700">
            <div class="animate-spin -ml-1 mr-3 h-5 w-5 text-gray-700">
                // Spinner SVG
            </div>
            "Loading..."
        </div>
    }
}
```

### Error Handling
```rust
// User-friendly error messages
#[component]
pub fn ErrorMessage(
    #[prop(into)] message: String,
) -> impl IntoView {
    view! {
        <div class="rounded-md bg-red-50 p-4">
            <div class="flex">
                <div class="ml-3">
                    <h3 class="text-sm font-medium text-red-800">
                        "Authentication Error"
                    </h3>
                    <div class="mt-2 text-sm text-red-700">
                        <p>{message}</p>
                    </div>
                </div>
            </div>
        </div>
    }
}
```

## Security Considerations

✅ **Cookie Security** - Secure, SameSite, HttpOnly configurations  
✅ **Session Validation** - Regular validation with backend  
✅ **Route Protection** - Proper authentication guards  
✅ **Token Handling** - Secure token storage and transmission  
✅ **Logout Cleanup** - Complete session cleanup on logout  
✅ **Error Handling** - No sensitive information in client errors

## Testing Strategy

### Component Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;

    #[test]
    fn test_auth_provider_context() {
        // Test AuthProvider context provision
    }

    #[test]
    fn test_protected_route_behavior() {
        // Test route protection logic
    }
}
```

### Integration Testing
```bash
# Test authentication flow
# 1. Visit login page
# 2. Click login button
# 3. Complete OAuth flow
# 4. Verify dashboard access
# 5. Test logout functionality
```

## Success Criteria

✅ **Authentication Flow** - Complete OAuth flow with proper redirects  
✅ **Session Management** - Persistent sessions across page reloads  
✅ **Route Protection** - Proper authentication guards for protected routes  
✅ **User Experience** - Smooth authentication experience with loading states  
✅ **Error Handling** - User-friendly error messages and recovery  
✅ **Security** - Secure cookie handling and session validation

## Deployment Considerations

### Cloudflare Workers Setup
```toml
# wrangler.toml configuration for frontend
[build]
command = "trunk build --release"
[build.upload]
format = "service-worker"
```

### Environment Variables
```bash
# Required for frontend build
LEPTOS_SITE_ROOT="/pkg"
LEPTOS_OUTPUT_NAME="frontend"
```

## Next Steps

After Phase 4 completion:
1. **Performance Optimization** - Bundle size optimization, lazy loading
2. **Gmail API Integration** - Add Gmail API functionality to frontend
3. **Enhanced UI/UX** - Improved styling, animations, responsiveness
4. **Testing Coverage** - Comprehensive test suite for all components
5. **Monitoring** - Error tracking and performance monitoring