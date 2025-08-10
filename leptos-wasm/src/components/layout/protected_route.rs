#![allow(dead_code)]
use leptos::prelude::*;
use crate::components::auth::use_auth_state;
use crate::types::auth::AuthState;

/// Hook that returns authentication state for protected routes
/// Returns: (should_show_loading, should_redirect_to_login, is_authenticated)
pub fn use_protected_route() -> Signal<(bool, bool, bool)> {
    let auth_state = use_auth_state();
    
    Signal::derive(move || {
        match auth_state.get() {
            AuthState::Loading => (true, false, false),
            AuthState::Authenticated(_) => (false, false, true),
            AuthState::Unauthenticated => (false, true, false),
        }
    })
}

/// Hook that returns authentication state for unauthenticated-only routes  
/// Returns: (should_show_loading, should_redirect_to_dashboard, is_unauthenticated)
pub fn use_unauthenticated_route() -> Signal<(bool, bool, bool)> {
    let auth_state = use_auth_state();
    
    Signal::derive(move || {
        match auth_state.get() {
            AuthState::Loading => (true, false, false),
            AuthState::Authenticated(_) => (false, true, false),
            AuthState::Unauthenticated => (false, false, true),
        }
    })
}


#[allow(dead_code)]
#[component]
pub fn LoadingSpinner(
    #[prop(optional)] class: Option<&'static str>,
    #[prop(optional)] text: Option<&'static str>,
) -> impl IntoView {
    let spinner_classes = format!(
        "animate-spin rounded-full border-4 border-blue-500 border-t-transparent {}",
        class.unwrap_or("h-8 w-8")
    );
    let loading_text = text.unwrap_or("Loading...");

    view! {
        <div class="flex items-center space-x-2">
            <div class=spinner_classes></div>
            <span class="text-gray-600">{loading_text}</span>
        </div>
    }
}

#[allow(dead_code)]
#[component]
pub fn FullPageLoader(
    #[prop(optional)] text: Option<&'static str>,
) -> impl IntoView {
    let loading_text = text.unwrap_or("Loading...");

    view! {
        <div class="fixed inset-0 bg-white bg-opacity-75 flex justify-center items-center z-50">
            <div class="flex flex-col items-center space-y-4">
                <div class="animate-spin rounded-full h-16 w-16 border-4 border-blue-500 border-t-transparent"></div>
                <p class="text-lg text-gray-700">{loading_text}</p>
            </div>
        </div>
    }
}