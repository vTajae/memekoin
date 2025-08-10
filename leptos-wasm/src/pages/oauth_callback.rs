use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_query_map;
use web_sys::window;

/// OAuth callback page that handles authorization code from Google  
#[component]
pub fn OAuthCallbackPage() -> impl IntoView {
    let query_map = use_query_map();
    let (callback_status, set_callback_status) = signal("Processing authentication...".to_string());
    let (is_error, set_is_error) = signal(false);

    // Handle OAuth callback when component mounts
    Effect::new(move |_| {
        spawn_local(async move {
            let code = query_map.with(|params| params.get("code").map(|s| s.clone()));
            let state = query_map.with(|params| params.get("state").map(|s| s.clone()));
            let error = query_map.with(|params| params.get("error").map(|s| s.clone()));

            // Check for OAuth error
            if let Some(error_msg) = error {
                log::error!("🔐 OAuth Error: {}", error_msg);
                set_callback_status.set(format!("Authentication failed: {}", error_msg));
                set_is_error.set(true);
                return;
            }

            // Ensure we have required parameters
            let (code, state) = match (code, state) {
                (Some(c), Some(s)) => (c, s),
                _ => {
                    log::error!("🔐 Missing OAuth parameters");
                    set_callback_status.set("Missing required authentication parameters".to_string());
                    set_is_error.set(true);
                    return;
                }
            };

            log::info!("🔐 Frontend: Processing OAuth callback with code and state");

            // Handle OAuth callback by submitting authorization code to backend
            use crate::services::auth_service::AuthService;
            let auth_service = AuthService::new();
            
            match auth_service.handle_oauth_callback(code, state).await {
                Ok(oauth_response) => {
                    log::info!("🔐 Frontend: OAuth callback successful, user: {}", oauth_response.user_email);
                    set_callback_status.set("Authentication successful! Redirecting...".to_string());
                    
                    // Redirect to dashboard
                    if let Some(window) = window() {
                        let location = window.location();
                        let _ = location.set_href("/dashboard");
                    }
                }
                Err(e) => {
                    log::error!("🔐 Frontend: OAuth callback failed: {}", e);
                    set_callback_status.set(format!("Authentication failed: {}", e));
                    set_is_error.set(true);
                }
            }
        });
    });

    view! {
        <div class="min-h-screen flex items-center justify-center bg-gray-50">
            <div class="max-w-md w-full space-y-8">
                <div class="text-center">
                    {move || if is_error.get() {
                        view! {
                            <div class="mx-auto h-12 w-12 text-red-500">
                                <svg fill="none" stroke="currentColor" viewBox="0 0 48 48" aria-hidden="true">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                                          d="M12 9v3m0 0v3m0-3h3m-3 0H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z" />
                                </svg>
                            </div>
                            <h2 class="mt-6 text-center text-3xl font-extrabold text-red-900">
                                "Authentication Failed"
                            </h2>
                            <p class="mt-2 text-center text-sm text-red-600">
                                {callback_status.get()}
                            </p>
                            <div class="mt-6">
                                <a
                                    href="/login"
                                    class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-red-600 hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500"
                                >
                                    "Try Again"
                                </a>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="mx-auto h-12 w-12">
                                <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
                            </div>
                            <h2 class="mt-6 text-center text-3xl font-extrabold text-gray-900">
                                "Redirecting..."
                            </h2>
                            <p class="mt-2 text-center text-sm text-gray-600">
                                {callback_status.get()}
                            </p>
                        }.into_any()
                    }}
                </div>
            </div>
        </div>
    }
}