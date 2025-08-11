use crate::components::counter_btn::Button;
use crate::context::auth::use_auth_state;
use crate::components::auth::user_profile::UserProfile;
use crate::types::auth::AuthState;
use leptos::prelude::*;
use leptos_router::{hooks::use_navigate, NavigateOptions};

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let auth_state = use_auth_state();
    let navigate = use_navigate();

    view! {
        <div class="min-h-screen bg-gray-50">
            // Header
            <header class="bg-white shadow-sm">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="flex justify-between items-center h-16">
                        <div class="flex items-center">
                            <h1 class="text-xl font-semibold text-gray-900">
                                "Leptos Cloudflare Template"
                            </h1>
                        </div>
                        <div class="flex items-center space-x-4">
                            <UserProfile />
                        </div>
                    </div>
                </div>
            </header>

            <ErrorBoundary fallback=|errors| {
                view! {
                    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
                        <div class="bg-red-50 border border-red-200 rounded-lg p-6">
                            <h1 class="text-lg font-medium text-red-800 mb-4">"Uh oh! Something went wrong!"</h1>
                            <div class="text-sm text-red-700">
                                <p class="mb-2">"Errors:"</p>
                                <ul class="list-disc list-inside space-y-1">
                                    {move || {
                                        errors
                                            .get()
                                            .into_iter()
                                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                            .collect_view()
                                    }}
                                </ul>
                            </div>
                        </div>
                    </div>
                }
            }>
                // Main content
                <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
                    <div class="text-center">
                        <picture class="flex justify-center mb-8">
                            <source
                                srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_pref_dark_RGB.svg"
                                media="(prefers-color-scheme: dark)"
                            />
                            <img
                                src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg"
                                alt="Leptos Logo"
                                height="200"
                                width="400"
                                class="h-32 w-auto"
                            />
                        </picture>

                        <h1 class="text-4xl font-bold text-gray-900 mb-4">
                            "Welcome to Leptos"
                        </h1>
                        
                        <p class="text-xl text-gray-600 mb-8 max-w-2xl mx-auto">
                            "A modern fullstack Rust web framework with Google OAuth authentication, powered by Cloudflare Workers."
                        </p>

                        // Authentication-aware action buttons
                        <div class="flex flex-col sm:flex-row gap-4 justify-center mb-12">
                            {
                            let navigate = navigate.clone();
                            move || match auth_state.get() {
                                AuthState::Authenticated(_) => view! {
                                    <button
                                        class="inline-flex items-center px-6 py-3 border border-transparent text-base font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                                        on:click={
                                            let navigate = navigate.clone();
                                            move |_| {
                                                navigate("/dashboard", NavigateOptions::default());
                                            }
                                        }
                                    >
                                        "Go to Dashboard"
                                    </button>
                                }.into_any(),
                                _ => view! {
                                    <a 
                                        href="/login"
                                        class="inline-flex items-center px-6 py-3 border border-transparent text-base font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                                    >
                                        "Get Started"
                                    </a>
                                }.into_any(),
                            }
                        }
                            
                            <a 
                                href="https://github.com/leptos-rs/leptos"
                                target="_blank"
                                class="inline-flex items-center px-6 py-3 border border-gray-300 text-base font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                            >
                                "Learn More"
                            </a>
                        </div>

                        // Demo section with counter buttons
                        <div class="bg-white rounded-lg shadow-lg p-8 max-w-md mx-auto mb-12">
                            <h2 class="text-2xl font-bold text-gray-900 mb-6">
                                "Interactive Demo"
                            </h2>
                            <div class="space-y-4">
                                <Button />
                                <Button increment=5 />
                            </div>
                        </div>

                        // Features section
                        <div class="grid grid-cols-1 md:grid-cols-3 gap-8 max-w-6xl mx-auto">
                            <div class="text-center p-6">
                                <div class="w-12 h-12 bg-blue-100 rounded-lg flex items-center justify-center mx-auto mb-4">
                                    <svg class="w-6 h-6 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
                                    </svg>
                                </div>
                                <h3 class="text-lg font-semibold text-gray-900 mb-2">"Lightning Fast"</h3>
                                <p class="text-gray-600">"Built with Rust and WASM for optimal performance on Cloudflare's edge network."</p>
                            </div>
                            
                            <div class="text-center p-6">
                                <div class="w-12 h-12 bg-green-100 rounded-lg flex items-center justify-center mx-auto mb-4">
                                    <svg class="w-6 h-6 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
                                    </svg>
                                </div>
                                <h3 class="text-lg font-semibold text-gray-900 mb-2">"Secure Auth"</h3>
                                <p class="text-gray-600">"Google OAuth integration with secure session management and CSRF protection."</p>
                            </div>
                            
                            <div class="text-center p-6">
                                <div class="w-12 h-12 bg-purple-100 rounded-lg flex items-center justify-center mx-auto mb-4">
                                    <svg class="w-6 h-6 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M9 19l3 3m0 0l3-3m-3 3V10" />
                                    </svg>
                                </div>
                                <h3 class="text-lg font-semibold text-gray-900 mb-2">"Edge Ready"</h3>
                                <p class="text-gray-600">"Deploy globally with zero configuration on Cloudflare Workers infrastructure."</p>
                            </div>
                        </div>
                    </div>
                </main>
            </ErrorBoundary>
        </div>
    }
}
