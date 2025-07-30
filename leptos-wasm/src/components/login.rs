use leptos::prelude::*;
use crate::auth::AuthContext;
use leptos_router::hooks::use_navigate;
use leptos::task::spawn_local;
use leptos::wasm_bindgen::JsCast;

#[component]
pub fn LoginForm() -> impl IntoView {
    let auth = expect_context::<AuthContext>();
    let navigate = use_navigate();
    
    let (username, set_username) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (show_password, set_show_password) = signal(false);
    let (remember_me, set_remember_me) = signal(false);
    let (error_message, set_error_message) = signal(Option::<String>::None);
    let (is_loading, set_is_loading) = signal(false);

    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        
        let username_val = username.get();
        let password_val = password.get();
        
        if username_val.is_empty() || password_val.is_empty() {
            set_error_message.set(Some("Please fill in all fields".to_string()));
            return;
        }

        set_is_loading.set(true);
        set_error_message.set(None);
        
        let nav = navigate.clone();
        spawn_local(async move {
            match crate::auth::login(username_val, password_val, auth.set_state).await {
                Ok(_) => {
                    // Login successful, navigate to dashboard
                    nav("/dashboard", Default::default());
                }
                Err(e) => {
                    set_error_message.set(Some(e));
                }
            }
            set_is_loading.set(false);
        });
    };

    view! {
        <div class="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
            <div class="max-w-md w-full space-y-8">
                <div>
                    <h2 class="mt-6 text-center text-3xl font-extrabold text-gray-900">
                        "Sign in to your account"
                    </h2>
                </div>
                <form class="mt-8 space-y-6" on:submit=on_submit>
                    <div class="rounded-md shadow-sm -space-y-px">
                        <div>
                            <label for="username" class="sr-only">
                                "Username"
                            </label>
                            <input
                                id="username"
                                name="username"
                                type="text"
                                required
                                class="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-t-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm"
                                placeholder="Username"
                                prop:value=username
                                on:input=move |ev| {
                                    set_username.set(event_target_value(&ev));
                                }
                            />
                        </div>
                        <div class="relative">
                            <label for="password" class="sr-only">
                                "Password"
                            </label>
                            <input
                                id="password"
                                name="password"
                                type=move || if show_password.get() { "text" } else { "password" }
                                required
                                class="appearance-none rounded-none relative block w-full px-3 py-2 pr-10 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-b-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm"
                                placeholder="Password"
                                prop:value=password
                                on:input=move |ev| {
                                    set_password.set(event_target_value(&ev));
                                }
                            />
                            <button
                                type="button"
                                class="absolute inset-y-0 right-0 pr-3 flex items-center"
                                on:click=move |_| set_show_password.update(|v| *v = !*v)
                            >
                                {move || if show_password.get() {
                                    view! {
                                        <svg class="h-5 w-5 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.88 9.88l-3.29-3.29m7.532 7.532l3.29 3.29M3 3l3.59 3.59m0 0A9.953 9.953 0 0112 5c4.478 0 8.268 2.943 9.543 7a10.025 10.025 0 01-4.132 5.411m0 0L21 21" />
                                        </svg>
                                    }.into_any()
                                } else {
                                    view! {
                                        <svg class="h-5 w-5 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                                        </svg>
                                    }.into_any()
                                }}
                            </button>
                        </div>
                    </div>

                    {move || error_message.get().map(|msg| view! {
                        <div class="rounded-md bg-red-50 p-4">
                            <div class="flex">
                                <div class="ml-3">
                                    <h3 class="text-sm font-medium text-red-800">
                                        "Error"
                                    </h3>
                                    <div class="mt-2 text-sm text-red-700">
                                        <p>{msg}</p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    })}

                    <div class="flex items-center justify-between">
                        <div class="flex items-center">
                            <input
                                id="remember-me"
                                name="remember-me"
                                type="checkbox"
                                class="h-4 w-4 text-indigo-600 focus:ring-indigo-500 border-gray-300 rounded"
                                prop:checked=remember_me
                                on:change=move |ev| {
                                    let target = ev.target().unwrap();
                                    let input = target.dyn_into::<web_sys::HtmlInputElement>().unwrap();
                                    set_remember_me.set(input.checked());
                                }
                            />
                            <label for="remember-me" class="ml-2 block text-sm text-gray-900">
                                "Remember me"
                            </label>
                        </div>

                        <div class="text-sm">
                            <a href="#" class="font-medium text-indigo-600 hover:text-indigo-500">
                                "Forgot your password?"
                            </a>
                        </div>
                    </div>

                    <div>
                        <button
                            type="submit"
                            disabled=move || is_loading.get()
                            class="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed"
                        >
                            {move || if is_loading.get() {
                                view! {
                                    <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                                    </svg>
                                    "Signing in..."
                                }.into_any()
                            } else {
                                view! { "Sign in" }.into_any()
                            }}
                        </button>
                    </div>

                    <div class="text-center space-y-4">
                        <p class="text-sm text-gray-600">
                            "Don't have an account? "
                            <a href="/register" class="font-medium text-indigo-600 hover:text-indigo-500">
                                "Sign up"
                            </a>
                        </p>
                        <div class="bg-blue-50 p-4 rounded-md">
                            <p class="text-sm text-blue-800">
                                <strong>"Demo credentials:"</strong>
                                <br />
                                "Username: demo"
                                <br />
                                "Password: password"
                            </p>
                        </div>
                    </div>
                </form>
            </div>
        </div>
    }
}

#[component]
pub fn RegisterForm() -> impl IntoView {
    let auth = expect_context::<AuthContext>();
    let navigate = use_navigate();
    
    let (username, set_username) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (confirm_password, set_confirm_password) = signal(String::new());
    let (error_message, set_error_message) = signal(Option::<String>::None);
    let (is_loading, set_is_loading) = signal(false);

    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        
        let username_val = username.get();
        let email_val = email.get();
        let password_val = password.get();
        let confirm_password_val = confirm_password.get();
        
        if username_val.is_empty() || email_val.is_empty() || password_val.is_empty() {
            set_error_message.set(Some("Please fill in all fields".to_string()));
            return;
        }

        if password_val != confirm_password_val {
            set_error_message.set(Some("Passwords do not match".to_string()));
            return;
        }

        if password_val.len() < 6 {
            set_error_message.set(Some("Password must be at least 6 characters long".to_string()));
            return;
        }

        set_is_loading.set(true);
        set_error_message.set(None);
        
        let nav = navigate.clone();
        spawn_local(async move {
            match crate::auth::register(username_val, email_val, password_val, auth.set_state).await {
                Ok(_) => {
                    // Registration successful, navigate to dashboard
                    nav("/dashboard", Default::default());
                }
                Err(e) => {
                    set_error_message.set(Some(e));
                }
            }
            set_is_loading.set(false);
        });
    };

    view! {
        <div class="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
            <div class="max-w-md w-full space-y-8">
                <div>
                    <h2 class="mt-6 text-center text-3xl font-extrabold text-gray-900">
                        "Create your account"
                    </h2>
                </div>
                <form class="mt-8 space-y-6" on:submit=on_submit>
                    <div class="rounded-md shadow-sm -space-y-px">
                        <div>
                            <label for="username" class="sr-only">
                                "Username"
                            </label>
                            <input
                                id="username"
                                name="username"
                                type="text"
                                required
                                class="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-t-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm"
                                placeholder="Username"
                                prop:value=username
                                on:input=move |ev| {
                                    set_username.set(event_target_value(&ev));
                                }
                            />
                        </div>
                        <div>
                            <label for="email" class="sr-only">
                                "Email"
                            </label>
                            <input
                                id="email"
                                name="email"
                                type="email"
                                required
                                class="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm"
                                placeholder="Email address"
                                prop:value=email
                                on:input=move |ev| {
                                    set_email.set(event_target_value(&ev));
                                }
                            />
                        </div>
                        <div>
                            <label for="password" class="sr-only">
                                "Password"
                            </label>
                            <input
                                id="password"
                                name="password"
                                type="password"
                                required
                                class="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm"
                                placeholder="Password"
                                prop:value=password
                                on:input=move |ev| {
                                    set_password.set(event_target_value(&ev));
                                }
                            />
                        </div>
                        <div>
                            <label for="confirm-password" class="sr-only">
                                "Confirm Password"
                            </label>
                            <input
                                id="confirm-password"
                                name="confirm-password"
                                type="password"
                                required
                                class="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-b-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm"
                                placeholder="Confirm Password"
                                prop:value=confirm_password
                                on:input=move |ev| {
                                    set_confirm_password.set(event_target_value(&ev));
                                }
                            />
                        </div>
                    </div>

                    {move || error_message.get().map(|msg| view! {
                        <div class="rounded-md bg-red-50 p-4">
                            <div class="flex">
                                <div class="ml-3">
                                    <h3 class="text-sm font-medium text-red-800">
                                        "Error"
                                    </h3>
                                    <div class="mt-2 text-sm text-red-700">
                                        <p>{msg}</p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    })}

                    <div>
                        <button
                            type="submit"
                            disabled=move || is_loading.get()
                            class="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed"
                        >
                            {move || if is_loading.get() {
                                view! {
                                    <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                                    </svg>
                                    "Creating account..."
                                }.into_any()
                            } else {
                                view! { "Create account" }.into_any()
                            }}
                        </button>
                    </div>

                    <div class="text-center">
                        <p class="text-sm text-gray-600">
                            "Already have an account? "
                            <a href="/login" class="font-medium text-indigo-600 hover:text-indigo-500">
                                "Sign in"
                            </a>
                        </p>
                    </div>
                </form>
            </div>
        </div>
    }
}
