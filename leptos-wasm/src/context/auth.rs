#![allow(dead_code)]
use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::services::auth::AuthService;
use crate::types::auth::{AuthState, User};
use tracing::{info, warn, error};

/// Authentication context for managing global auth state
#[derive(Clone)]
pub struct AuthContext {
    pub state: ReadSignal<AuthState>,
    pub login: Callback<()>,
    pub logout: Callback<()>,
    pub refresh_session: Callback<()>,
}

/// Authentication provider component that manages auth state globally
#[component]
pub fn AuthProvider(children: Children) -> impl IntoView {
    let auth_service = AuthService::new();

    // Create reactive auth state
    let (auth_state, set_auth_state) = signal(AuthState::Loading);

    // Session management via backend API calls
    // The reqwest client automatically handles cookies sent by the backend

    // Initialize authentication state on mount
    Effect::new({
        let auth_service = auth_service.clone();
        let set_auth_state = set_auth_state.clone();
        
        move |_| {
            let auth_service = auth_service.clone();
            let set_auth_state = set_auth_state.clone();
            
            spawn_local(async move {
                // Try to validate session - cookies are sent automatically by reqwest
                match auth_service.validate_session().await {
                    Ok(user) => {
                        info!("AuthProvider: session valid; authenticated user");
                        set_auth_state.set(AuthState::Authenticated(user));
                    }
                    Err(e) => {
                        warn!("AuthProvider: no valid session ({})", e);
                        // No valid session found
                        set_auth_state.set(AuthState::Unauthenticated);
                    }
                }
            });
        }
    });

    // Login callback - redirects to OAuth provider
    let login_callback = {
        let auth_service = auth_service.clone();
        
        Callback::new(move |_: ()| {
            let auth_service = auth_service.clone();
            spawn_local(async move {
                if let Err(e) = auth_service.initiate_oauth_login() {
                    error!("AuthProvider: failed to initiate OAuth login: {}", e);
                }
            });
        })
    };

    // Logout callback - clears session and updates state
    let logout_callback = {
        let auth_service = auth_service.clone();
        let set_auth_state = set_auth_state.clone();
        
        Callback::new(move |_: ()| {
            let auth_service = auth_service.clone();
            let set_auth_state = set_auth_state.clone();
            
            spawn_local(async move {
                // Call logout API
                if let Err(e) = auth_service.logout().await {
                    warn!("AuthProvider: logout API call failed: {}", e);
                } else {
                    info!("AuthProvider: logout succeeded");
                }

                // Session cookie will be cleared by backend logout endpoint
                set_auth_state.set(AuthState::Unauthenticated);
            });
        })
    };

    // Refresh session callback
    let refresh_session_callback = {
        let auth_service = auth_service.clone();
        let set_auth_state = set_auth_state.clone();
        
        Callback::new(move |_: ()| {
            let auth_service = auth_service.clone();
            let set_auth_state = set_auth_state.clone();
            
            spawn_local(async move {
                match auth_service.validate_session().await {
                    Ok(user) => {
                        info!("AuthProvider: refresh_session validated");
                        set_auth_state.set(AuthState::Authenticated(user));
                    }
                    Err(e) => {
                        warn!("AuthProvider: refresh_session invalid: {}", e);
                        // Session is invalid
                        set_auth_state.set(AuthState::Unauthenticated);
                    }
                }
            });
        })
    };

    // Create and provide auth context
    let auth_context = AuthContext {
        state: auth_state.into(),
        login: login_callback,
        logout: logout_callback,
        refresh_session: refresh_session_callback,
    };

    provide_context(auth_context);

    children()
}

/// Hook to use authentication context
pub fn use_auth() -> AuthContext {
    use_context::<AuthContext>()
        .expect("AuthContext must be provided by AuthProvider")
}

/// Hook to use authentication context (alias for use_auth)
pub fn use_auth_context() -> Option<AuthContext> {
    use_context::<AuthContext>()
}

/// Hook to get current authentication state
pub fn use_auth_state() -> ReadSignal<AuthState> {
    use_auth().state
}

/// Hook to get current user if authenticated
pub fn use_current_user() -> Signal<Option<User>> {
    let auth_state = use_auth_state();
    Signal::derive(move || match auth_state.get() {
        AuthState::Authenticated(user) => Some(user),
        _ => None,
    })
}

/// Hook to check if user is authenticated
pub fn use_is_authenticated() -> Signal<bool> {
    let auth_state = use_auth_state();
    Signal::derive(move || auth_state.get().is_authenticated())
}