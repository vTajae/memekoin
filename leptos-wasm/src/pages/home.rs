use crate::components::counter_btn::Button;
use crate::components::market_data::MarketDataDashboard;
use crate::auth::{AuthContext, AuthState};
use leptos::prelude::*;
use leptos_router::components::A;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let auth = expect_context::<AuthContext>();
    
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

            <div class="min-h-screen bg-gradient-to-br from-gray-50 to-gray-100">
                <div class="container mx-auto px-4 py-16">
                    <div class="text-center mb-12">
                        <picture class="inline-block mb-8">
                            <source
                                srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_pref_dark_RGB.svg"
                                media="(prefers-color-scheme: dark)"
                            />
                            <img
                                src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg"
                                alt="Leptos Logo"
                                height="150"
                                width="300"
                                class="mx-auto"
                            />
                        </picture>

                        <h1 class="text-4xl md:text-6xl font-bold text-gray-900 mb-4">
                            "Welcome to Leptos"
                        </h1>
                        
                        <p class="text-lg md:text-xl text-gray-600 mb-8 max-w-2xl mx-auto">
                            "Build fast, reactive web applications with Rust and WebAssembly"
                        </p>

                        {move || match auth.state.get() {
                            AuthState::Loading => view! {
                                <div class="animate-pulse">
                                    <div class="h-12 w-48 bg-gray-200 rounded-lg mx-auto"></div>
                                </div>
                            }.into_any(),
                            AuthState::Authenticated(user) => view! {
                                <div class="space-y-4">
                                    <p class="text-2xl text-gray-700 mb-6">
                                        "Welcome back, " <span class="font-semibold text-indigo-600">{user.username}</span> "!"
                                    </p>
                                    <A 
                                        href="/dashboard"
                                        attr:class="inline-block bg-indigo-600 hover:bg-indigo-700 text-white px-8 py-3 rounded-lg text-lg font-medium transition-colors duration-200 shadow-lg hover:shadow-xl"
                                    >
                                        "Go to Dashboard"
                                    </A>
                                </div>
                            }.into_any(),
                            AuthState::Unauthenticated => view! {
                                <div class="space-x-4">
                                    <A 
                                        href="/login"
                                        attr:class="inline-block bg-white hover:bg-gray-50 text-gray-800 px-8 py-3 rounded-lg text-lg font-medium transition-colors duration-200 shadow-md hover:shadow-lg border border-gray-200"
                                    >
                                        "Sign In"
                                    </A>
                                    <A 
                                        href="/register"
                                        attr:class="inline-block bg-indigo-600 hover:bg-indigo-700 text-white px-8 py-3 rounded-lg text-lg font-medium transition-colors duration-200 shadow-lg hover:shadow-xl"
                                    >
                                        "Get Started"
                                    </A>
                                </div>
                            }.into_any(),
                        }}
                    </div>

                    <div class="grid md:grid-cols-3 gap-8 mt-16 max-w-4xl mx-auto">
                        <div class="bg-white p-6 rounded-lg shadow-md">
                            <h3 class="text-xl font-semibold text-gray-900 mb-2">
                                "🚀 Fast Performance"
                            </h3>
                            <p class="text-gray-600">
                                "Compiled to WebAssembly for near-native performance in the browser"
                            </p>
                        </div>
                        <div class="bg-white p-6 rounded-lg shadow-md">
                            <h3 class="text-xl font-semibold text-gray-900 mb-2">
                                "🦀 Type Safety"
                            </h3>
                            <p class="text-gray-600">
                                "Leverage Rust's powerful type system for reliable web applications"
                            </p>
                        </div>
                        <div class="bg-white p-6 rounded-lg shadow-md">
                            <h3 class="text-xl font-semibold text-gray-900 mb-2">
                                "⚡ Reactive"
                            </h3>
                            <p class="text-gray-600">
                                "Fine-grained reactivity system for efficient UI updates"
                            </p>
                        </div>
                    </div>

                    <div class="mt-16 text-center">
                        <h2 class="text-2xl font-semibold text-gray-900 mb-6">"Try the Counter Demo"</h2>
                        <div class="flex justify-center space-x-4">
                            <Button />
                            <Button increment=5 />
                        </div>
                    </div>

                    // Market Data Dashboard Section
                    <div class="mt-16">
                        <MarketDataDashboard />
                    </div>
                </div>
            </div>
        </ErrorBoundary>
    }
}
