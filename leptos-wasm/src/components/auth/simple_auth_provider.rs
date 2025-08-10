use leptos::prelude::*;
use leptos::task::spawn_local;
use web_sys::window;
use crate::services::auth_service::AuthService;
use crate::types::auth::{AuthState, User};

/// Simple authentication context for managing global auth state
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
    // Create reactive auth state
    let (auth_state, set_auth_state) = signal(AuthState::Loading);

    // Helper functions for localStorage access
    let get_session_id = || -> Option<String> {
        window().and_then(|w| w.local_storage().ok().flatten())
            .and_then(|storage| storage.get_item("session_id").ok().flatten())
    };

    // Initialize authentication state on mount
    {
        let set_auth_state = set_auth_state.clone();
        Effect::new(move |_| {
            if let Some(_session_id) = get_session_id() {
                let auth_service = AuthService::new();
                let set_auth_state = set_auth_state.clone();
                
                spawn_local(async move {
                    match auth_service.validate_session().await {
                        Ok(user) => {
                            set_auth_state.set(AuthState::Authenticated(user));
                        }
                        Err(_) => {
                            // Clear invalid session
                            if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten()) {
                                let _ = storage.remove_item("session_id");
                            }
                            set_auth_state.set(AuthState::Unauthenticated);
                        }
                    }
                });
            } else {
                set_auth_state.set(AuthState::Unauthenticated);
            }
        });
    }

    // Login callback - redirects to OAuth provider
    let login_callback = Callback::new(move |_: ()| {
        let auth_service = AuthService::new();
        if let Err(e) = auth_service.initiate_oauth_login() {
            log::error!("Failed to initiate OAuth login: {}", e);
        }
    });

    // Logout callback - clears session and updates state
    let logout_callback = {
        let set_auth_state = set_auth_state.clone();
        
        Callback::new(move |_: ()| {
            let auth_service = AuthService::new();
            let set_auth_state = set_auth_state.clone();
            
            spawn_local(async move {
                // Call logout API
                if let Err(e) = auth_service.logout().await {
                    log::warn!("Logout API call failed: {}", e);
                }
                
                // Clear local state regardless of API success
                if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten()) {
                    let _ = storage.remove_item("session_id");
                }
                set_auth_state.set(AuthState::Unauthenticated);
            });
        })
    };

    // Refresh session callback
    let refresh_session_callback = {
        let set_auth_state = set_auth_state.clone();
        
        Callback::new(move |_: ()| {
            let auth_service = AuthService::new();
            let set_auth_state = set_auth_state.clone();
            
            spawn_local(async move {
                match auth_service.validate_session().await {
                    Ok(user) => {
                        set_auth_state.set(AuthState::Authenticated(user));
                    }
                    Err(_) => {
                        // Session is invalid, clear it
                        if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten()) {
                            let _ = storage.remove_item("session_id");
                        }
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