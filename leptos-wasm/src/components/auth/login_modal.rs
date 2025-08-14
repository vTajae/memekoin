use leptos::prelude::*;
use leptos::html::Dialog;
use leptos::ev::SubmitEvent;
use leptos::task::spawn_local;
use leptos::callback::Callback;
use crate::services::axiom::{AxiomService, AxiomAuthRequest, AxiomUserData};
use web_sys::KeyboardEvent;

// Using structs from services::axiom module

#[derive(Debug, Clone, PartialEq)]
enum AuthStep {
    Email,
    Password,
    TwoFactor,
    Complete,
}

#[component]
pub fn LoginModal(
    /// Signal to control modal visibility
    show: ReadSignal<bool>,
    /// Callback to set modal visibility
    set_show: WriteSignal<bool>,
    /// Callback when authentication succeeds
    #[prop(optional)] on_success: Option<Callback<AxiomUserData>>,
) -> impl IntoView {
    let dialog_ref = NodeRef::<Dialog>::new();
    
    // State management for multi-step authentication
    let (current_step, set_current_step) = signal(AuthStep::Email);
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());
    // NOTE: session_id here stores the Axiom step1 JWT (otp token),
    // which we pass to step2 and the backend forwards as the
    // "auth-otp-login-token" cookie to Axiom.
    let (session_id, set_session_id) = signal::<Option<String>>(None);
    let (loading, set_loading) = signal(false);
    let (error_message, set_error_message) = signal::<Option<String>>(None);
    let (success_message, set_success_message) = signal::<Option<String>>(None);
    let (user_data, set_user_data) = signal::<Option<AxiomUserData>>(None);
    let (otp_code, set_otp_code) = signal(String::new());

    // Avoid unused warning if parent doesn't pass on_success
    let _ = &on_success;

    // Effect to open/close dialog based on show signal
    Effect::new(move || {
        if let Some(dialog) = dialog_ref.get() {
            if show.get() {
                let _ = dialog.show_modal();
                // Reset state when opening modal
                set_current_step.set(AuthStep::Email);
                set_email.set(String::new());
                set_password.set(String::new());
                set_session_id.set(None);
                set_error_message.set(None);
                set_success_message.set(None);
                set_user_data.set(None);
            } else {
                dialog.close();
            }
        }
    });

    // Keyboard handling is attached directly to dialog and inputs below.

    // Close modal when clicking backdrop
    let on_backdrop_click = move |e: web_sys::MouseEvent| {
        if let Some(dialog) = dialog_ref.get() {
            let rect = dialog.get_bounding_client_rect();
            let x = e.client_x() as f64;
            let y = e.client_y() as f64;
            
            // Check if click was outside the dialog content
            if x < rect.left() || x > rect.right() || y < rect.top() || y > rect.bottom() {
                set_show.set(false);
            }
        }
    };

    // Handle authentication step submission
    let handle_auth_step = move |step: AuthStep| {
        spawn_local(async move {
            set_loading.set(true);
            set_error_message.set(None);

            let axiom_service = AxiomService::new();

            match step {
                AuthStep::Email | AuthStep::Password => {
                    // Step 1: email+password
                    let request_body = AxiomAuthRequest {
                        step: "email".to_string(),
                        email: Some(email.get()),
                        password: Some(password.get()),
                        session_id: None,
                        otp_code: None,
                    };
                    match axiom_service.handle_auth_step(request_body).await {
                        Ok(auth_response) => {
                            if auth_response.success {
                                // Store the Axiom step1 JWT for step2 (auth-otp-login-token)
                                if let Some(jwt) = auth_response.axiom_jwt.clone() {
                                    set_session_id.set(Some(jwt.clone()));
                                }
                                if let Some(data) = auth_response.user_data {
                                    set_user_data.set(Some(data));
                                }
                                if let Some(msg) = auth_response.message {
                                    set_error_message.set(None);
                                    set_success_message.set(Some(msg));
                                }
                                match auth_response.next_step.as_deref() {
                                    Some("2fa") => {
                                        set_current_step.set(AuthStep::TwoFactor);
                                        set_error_message.set(None);
                                        set_success_message.set(Some("Retrieving 2FA code from Gmail...".to_string()));
                                        // Try to auto-fetch OTP from Gmail, then auto-submit step2
                                        let user_email = email.get();
                                        // Use backend-provided axiom_jwt (actual JWT)
                                        let jwt_opt = auth_response.axiom_jwt.clone();
                                        let set_otp_code = set_otp_code.clone();
                                        let set_loading = set_loading.clone();
                                        let set_error_message = set_error_message.clone();
                                        let set_success_message = set_success_message.clone();
                                        let set_session_id = set_session_id.clone();
                                        let set_user_data = set_user_data.clone();
                                        let set_current_step = set_current_step.clone();
                                        // Capture password for step2 (required by backend)
                                        let user_password_capture = password.get();
                                        spawn_local(async move {
                                            let axiom_service = AxiomService::new();
                                            match axiom_service.get_2fa_code_from_gmail(&user_email).await {
                                                Ok(code) => {
                                                    set_otp_code.set(code.clone());
                                                    set_error_message.set(None);
                                                    set_success_message.set(Some("2FA code retrieved from Gmail. Verifying...".to_string()));
                                                    if let Some(jwt) = jwt_opt {
                                                        // Submit step 2 via service orchestrator
                                                        let req = AxiomAuthRequest {
                                                            step: "2fa".to_string(),
                                                            email: Some(user_email.clone()),
                                                            password: Some(user_password_capture.clone()),
                                                            session_id: Some(jwt.clone()),
                                                            otp_code: Some(code.clone()),
                                                        };
                                                        match axiom_service.handle_auth_step(req).await {
                                                            Ok(step2_resp) => {
                                                                if step2_resp.success {
                                                                    if let Some(sid2) = step2_resp.session_id {
                                                                        set_session_id.set(Some(sid2));
                                                                    }
                                                                    if let Some(data) = step2_resp.user_data {
                                                                        set_user_data.set(Some(data));
                                                                    }
                                                                    if let Some(msg) = step2_resp.message {
                                                                        set_error_message.set(None);
                                                                        set_success_message.set(Some(msg));
                                                                    }
                                                                    set_current_step.set(AuthStep::Complete);
                                                                } else {
                                                                    set_success_message.set(None);
                                                                    set_error_message.set(step2_resp.message.or(Some("2FA verification failed".to_string())));
                                                                }
                                                                set_loading.set(false);
                                                            }
                                                            Err(e) => {
                                                                set_success_message.set(None);
                                                                set_error_message.set(Some(format!("2FA request failed: {}", e)));
                                                                set_loading.set(false);
                                                            }
                                                        }
                                                    } else {
                                                        set_success_message.set(None);
                                                        set_error_message.set(Some("Missing JWT from step1; please enter the 2FA code manually.".to_string()));
                                                        set_loading.set(false);
                                                    }
                                                }
                                                Err(e) => {
                                                    set_success_message.set(None);
                                                    set_error_message.set(Some(format!("Could not retrieve 2FA code automatically: {}. Please check your email and enter the code manually.", e)));
                                                    set_loading.set(false);
                                                }
                                            }
                                        });
                                    }
                                    _ => {
                                        // Shouldn't happen, but keep UI responsive
                                        set_loading.set(false);
                                    }
                                }
                            } else {
                                // Step 1 failed
                                set_success_message.set(None);
                                set_error_message.set(auth_response.message.or(Some("Authentication failed".to_string())));
                                set_loading.set(false);
                            }
                        }
                        Err(e) => {
                            set_success_message.set(None);
                            set_error_message.set(Some(format!("Network error: {}", e)));
                            set_loading.set(false);
                        }
                    }
                }
                AuthStep::TwoFactor => {
                    // Step 2: 2FA
                    // Enforce presence of axiom_jwt; do not use session_id fallback
                    if session_id.get().is_none() {
                        set_success_message.set(None);
                        set_error_message.set(Some("Missing Axiom JWT from step 1. Please retry login.".to_string()));
                        set_loading.set(false);
                        return;
                    }
                    // Ensure password is still present for step 2
                    if password.get().trim().is_empty() {
                        set_success_message.set(None);
                        set_error_message.set(Some("Password is required to complete 2FA.".to_string()));
                        set_loading.set(false);
                        return;
                    }
                    let request_body = AxiomAuthRequest {
                        step: "2fa".to_string(),
                        email: Some(email.get()),
                        // Pass password if present; else empty and let backend/session handle
                        password: Some(password.get()),
                        session_id: session_id.get(),
                        otp_code: Some(otp_code.get()),
                    };
                    match axiom_service.handle_auth_step(request_body).await {
                        Ok(auth_response) => {
                            if auth_response.success {
                                if let Some(sid) = auth_response.session_id {
                                    set_session_id.set(Some(sid));
                                }
                                if let Some(data) = auth_response.user_data {
                                    set_user_data.set(Some(data));
                                }
                                if let Some(msg) = auth_response.message {
                                    set_error_message.set(None);
                                    set_success_message.set(Some(msg));
                                }
                                set_current_step.set(AuthStep::Complete);
                                set_timeout(
                                    move || {
                                        set_show.set(false);
                                    },
                                    std::time::Duration::from_secs(3),
                                );
                            } else {
                                set_success_message.set(None);
                                set_error_message.set(auth_response.message);
                            }
                        }
                        Err(e) => {
                            set_success_message.set(None);
                            set_error_message.set(Some(format!("Network error: {}", e)));
                        }
                    }
                    set_loading.set(false);
                }
                _ => {
                    set_loading.set(false);
                }
            }
        });
    };

    // Now define keyboard handler that can call handle_auth_step
    let on_keydown = {
        let current_step = current_step.clone();
        let email = email.clone();
        let password = password.clone();
        let otp_code = otp_code.clone();
        let loading = loading.clone();
        let set_current_step = set_current_step.clone();
        let set_error_message = set_error_message.clone();
        move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                e.prevent_default();
                e.stop_propagation();
                if loading.get() { return; }
                match current_step.get() {
                    AuthStep::Email => {
                        if !email.get().is_empty() {
                            set_current_step.set(AuthStep::Password);
                            set_error_message.set(None);
                        }
                    }
                    AuthStep::Password => {
                        if !password.get().is_empty() {
                            handle_auth_step(AuthStep::Email);
                        }
                    }
                    AuthStep::TwoFactor => {
                        if !otp_code.get().is_empty() {
                            handle_auth_step(AuthStep::TwoFactor);
                        }
                    }
                    _ => {}
                }
            }
        }
    };

    // Handle email form submission - just move to password step
    let on_email_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        if !email.get().is_empty() {
            // Just move to password step after validating email
            set_current_step.set(AuthStep::Password);
            set_error_message.set(None);
            set_success_message.set(Some("Email accepted. Please enter your password.".to_string()));
        }
    };

    // Handle password form submission - now submit both email and password
    let on_password_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        if !password.get().is_empty() {
            // Now submit both email and password together in step 1
            handle_auth_step(AuthStep::Email); // This sends both email and password
        }
    };

    // Handle OTP form submission
    let on_otp_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        if !otp_code.get().is_empty() && session_id.get().is_some() {
            handle_auth_step(AuthStep::TwoFactor);
        }
    };

    view! {
        <dialog
            node_ref=dialog_ref
            class="backdrop:bg-gray-900/50 p-0 rounded-lg shadow-xl max-w-md w-full"
            on:click=on_backdrop_click
            on:keydown=on_keydown
        >
            <div class="bg-white rounded-lg">
                // Modal header with Axiom Trade branding
                <div class="flex items-center justify-between p-6 border-b border-gray-200 bg-gradient-to-r from-indigo-600 to-purple-600">
                    <div class="flex items-center space-x-3">
                        <svg class="w-8 h-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
                        </svg>
                        <h2 class="text-xl font-semibold text-white">
                            "Axiom Trade Login"
                        </h2>
                    </div>
                    <button
                        type="button"
                        class="text-white/80 hover:text-white focus:outline-none transition ease-in-out duration-150"
                        on:click=move |_| set_show.set(false)
                    >
                        <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                        </svg>
                    </button>
                </div>

                // Modal body with step-based content
                <div class="p-6">
                    {move || match current_step.get() {
                        AuthStep::Email => view! {
                            <div class="space-y-4">
                                <div class="text-center mb-4">
                                    <p class="text-gray-600 font-medium">
                                        "Step 1 of 3: Enter your Axiom Trade email"
                                    </p>
                                    <p class="text-xs text-gray-500 mt-1">
                                        "Please enter your email address to begin authentication"
                                    </p>
                                </div>
                                
                                <Show when=move || error_message.get().is_some()>
                                    <div class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded">
                                        {move || error_message.get()}
                                    </div>
                                </Show>
                                
                                <Show when=move || success_message.get().is_some() && error_message.get().is_none()>
                                    <div class="bg-green-50 border border-green-200 text-green-700 px-4 py-3 rounded">
                                        {move || success_message.get()}
                                    </div>
                                </Show>
                                
                                <form on:submit=on_email_submit>
                                    <div class="space-y-4">
                                        <div>
                                            <label for="email" class="block text-sm font-medium text-gray-700 mb-1">
                                                "Email Address"
                                            </label>
                                            <input
                                                type="email"
                                                id="email"
                                                class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
                                                placeholder="the.last.tajae@gmail.com"
                                                on:input=move |ev| {
                                                    set_email.set(event_target_value(&ev));
                                                }
                                                on:keydown=move |e: KeyboardEvent| {
                                                    if e.key() == "Enter" {
                                                        e.prevent_default();
                                                        e.stop_propagation();
                                                        if !loading.get() && !email.get().is_empty() {
                                                            set_current_step.set(AuthStep::Password);
                                                            set_error_message.set(None);
                                                        }
                                                    }
                                                }
                                                prop:value=move || email.get()
                                                required
                                                autofocus
                                            />
                                        </div>
                                        
                                        <button
                                            type="submit"
                                            class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-gradient-to-r from-indigo-600 to-purple-600 hover:from-indigo-700 hover:to-purple-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed transition-all"
                                            disabled=move || loading.get()
                                        >
                                            {move || if loading.get() {
                                                "Verifying..."
                                            } else {
                                                "Continue to Password →"
                                            }}
                                        </button>
                                    </div>
                                </form>
                            </div>
                        }.into_any(),
                        
                        AuthStep::Password => view! {
                            <div class="space-y-4">
                                <div class="text-center mb-4">
                                    <p class="text-gray-600 font-medium">
                                        "Step 2 of 3: Enter your password"
                                    </p>
                                    <p class="text-sm text-gray-500 mt-1">
                                        "Email: " <span class="font-medium text-gray-700">{move || email.get()}</span>
                                    </p>
                                    <p class="text-xs text-gray-500 mt-1">
                                        "Your password will be securely transmitted"
                                    </p>
                                </div>
                                
                                <Show when=move || error_message.get().is_some()>
                                    <div class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded">
                                        {move || error_message.get()}
                                    </div>
                                </Show>
                                
                                <Show when=move || success_message.get().is_some() && error_message.get().is_none()>
                                    <div class="bg-green-50 border border-green-200 text-green-700 px-4 py-3 rounded">
                                        {move || success_message.get()}
                                    </div>
                                </Show>
                                
                                <form on:submit=on_password_submit>
                                    <div class="space-y-4">
                                        <div>
                                            <label for="password" class="block text-sm font-medium text-gray-700 mb-1">
                                                "Password"
                                            </label>
                                            <input
                                                type="password"
                                                id="password"
                                                class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500"
                                                placeholder="Enter your Axiom Trade password"
                                                on:input=move |ev| {
                                                    set_password.set(event_target_value(&ev));
                                                }
                                                on:keydown=move |e: KeyboardEvent| {
                                                    if e.key() == "Enter" {
                                                        e.prevent_default();
                                                        e.stop_propagation();
                                                        if !loading.get() && !password.get().is_empty() {
                                                            handle_auth_step(AuthStep::Email);
                                                        }
                                                    }
                                                }
                                                prop:value=move || password.get()
                                                required
                                                autofocus
                                            />
                                        </div>
                                        
                                        <button
                                            type="submit"
                                            class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-gradient-to-r from-indigo-600 to-purple-600 hover:from-indigo-700 hover:to-purple-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed transition-all"
                                            disabled=move || loading.get()
                                        >
                                            {move || if loading.get() {
                                                "Authenticating..."
                                            } else {
                                                "Sign In"
                                            }}
                                        </button>
                                    </div>
                                </form>
                            </div>
                        }.into_any(),
                        AuthStep::TwoFactor => view! {
                            <div class="space-y-4">
                                <div class="text-center mb-4">
                                    <p class="text-gray-600 font-medium">
                                        "Step 3 of 3: Enter your 2FA code"
                                    </p>
                                    <p class="text-sm text-gray-500 mt-1">
                                        "We sent a 6-digit verification code to your email"
                                    </p>
                                </div>

                                <Show when=move || error_message.get().is_some()>
                                    <div class="bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded">
                                        {move || error_message.get()}
                                    </div>
                                </Show>

                                <Show when=move || success_message.get().is_some() && error_message.get().is_none()>
                                    <div class="bg-green-50 border border-green-200 text-green-700 px-4 py-3 rounded">
                                        {move || success_message.get()}
                                    </div>
                                </Show>

                                <form on:submit=on_otp_submit>
                                    <div class="space-y-4">
                                        <div>
                                            <label for="otp" class="block text-sm font-medium text-gray-700 mb-1">
                                                "2FA Code"
                                            </label>
                                            <input
                                                type="text"
                                                id="otp"
                                                maxlength="6"
                                                pattern="[0-9]{6}"
                                                inputmode="numeric"
                                                class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 tracking-widest text-center"
                                                placeholder="••••••"
                                                on:input=move |ev| {
                                                    set_otp_code.set(event_target_value(&ev));
                                                }
                                                on:keydown=move |e: KeyboardEvent| {
                                                    if e.key() == "Enter" {
                                                        e.prevent_default();
                                                        e.stop_propagation();
                                                        if !loading.get() && session_id.get().is_some() && !otp_code.get().is_empty() {
                                                            handle_auth_step(AuthStep::TwoFactor);
                                                        }
                                                    }
                                                }
                                                prop:value=move || otp_code.get()
                                                required
                                                autofocus
                                            />
                                        </div>

                                        <button
                                            type="submit"
                                            class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-gradient-to-r from-indigo-600 to-purple-600 hover:from-indigo-700 hover:to-purple-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed transition-all"
                                            disabled=move || loading.get() || session_id.get().is_none()
                                        >
                                            {move || if loading.get() { "Verifying..." } else { "Verify Code" }}
                                        </button>
                                    </div>
                                </form>
                            </div>
                        }.into_any(),
                        
                        AuthStep::Complete => view! {
                            <div class="space-y-4">
                                <div class="text-center">
                                    <div class="mx-auto flex items-center justify-center h-12 w-12 rounded-full bg-green-100 mb-4">
                                        <svg class="h-6 w-6 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                                        </svg>
                                    </div>
                                    <h3 class="text-lg font-medium text-gray-900">
                                        "Authentication Successful!"
                                    </h3>
                                    
                                    {move || match user_data.get() {
                                        Some(data) => view! {
                                            <div class="mt-4 text-sm text-gray-600">
                                                <p>"Welcome, " {data.name}</p>
                                                <p class="mt-1">"Account: " {data.account_id}</p>
                                                <p class="mt-1">
                                                    "Trading Status: "
                                                    <span class={if data.trading_enabled { "text-green-600" } else { "text-red-600" }}>
                                                        {if data.trading_enabled { "Enabled" } else { "Disabled" }}
                                                    </span>
                                                </p>
                                            </div>
                                        }.into_any(),
                                        None => view! { <div></div> }.into_any(),
                                    }}
                                    // Optionally notify parent via on_success callback
                                    // Parent callback can be wired here if needed
                                    
                                    <p class="text-sm text-gray-500 mt-4">
                                        "Closing in a moment..."
                                    </p>
                                </div>
                            </div>
                        }.into_any(),
                    }}
                    
                    // Progress indicator
                    <div class="mt-6 flex justify-center space-x-2">
                        <div class={move || format!(
                            "h-2 w-2 rounded-full {}",
                            if current_step.get() == AuthStep::Email { "bg-indigo-600" } else { "bg-gray-300" }
                        )} />
                        <div class={move || format!(
                            "h-2 w-2 rounded-full {}",
                            if current_step.get() == AuthStep::Password { "bg-indigo-600" } else { "bg-gray-300" }
                        )} />
                        <div class={move || format!(
                            "h-2 w-2 rounded-full {}",
                            if current_step.get() == AuthStep::TwoFactor || current_step.get() == AuthStep::Complete { "bg-indigo-600" } else { "bg-gray-300" }
                        )} />
                    </div>
                </div>
            </div>
        </dialog>
    }
}

// Helper function to set timeout
fn set_timeout<F>(callback: F, duration: std::time::Duration)
where
    F: FnOnce() + 'static,
{
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    
    let window = web_sys::window().unwrap();
    let closure = Closure::once_into_js(move || callback());
    
    window.set_timeout_with_callback_and_timeout_and_arguments_0(
        closure.as_ref().unchecked_ref(),
        duration.as_millis() as i32,
    ).unwrap();
}