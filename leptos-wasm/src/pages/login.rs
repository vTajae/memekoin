use leptos::prelude::*;
use leptos_router::components::Redirect;
use crate::components::auth::login_button::LoginButton;
use crate::components::layout::protected_route::use_unauthenticated_route;

#[component]
pub fn LoginPage() -> impl IntoView {
    let auth_guard = use_unauthenticated_route();
    
    view! {
        <Show
            when=move || {
                let (loading, _redirect, _unauthenticated) = auth_guard.get();
                loading
            }
            fallback=move || {
                let (_loading, redirect, unauthenticated) = auth_guard.get();
                if redirect {
                    view! { <Redirect path="/dashboard" /> }.into_any()
                } else if unauthenticated {
                    view! {
            <div class="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
                <div class="max-w-md w-full space-y-8">
                    <div class="text-center">
                        <div class="mx-auto h-12 w-12 flex items-center justify-center rounded-full bg-blue-100">
                            <svg 
                                class="h-6 w-6 text-blue-600" 
                                fill="none" 
                                stroke="currentColor" 
                                viewBox="0 0 24 24"
                            >
                                <path 
                                    stroke-linecap="round" 
                                    stroke-linejoin="round" 
                                    stroke-width="2" 
                                    d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
                                />
                            </svg>
                        </div>
                        <h2 class="mt-6 text-center text-3xl font-extrabold text-gray-900">
                            "Welcome Back"
                        </h2>
                        <p class="mt-2 text-center text-sm text-gray-600">
                            "Sign in to access your dashboard and manage your account"
                        </p>
                    </div>
                    
                    <div class="mt-8 space-y-6">
                        <div class="rounded-md shadow-sm">
                            <div class="bg-white px-6 py-8 rounded-lg border border-gray-200">
                                <div class="space-y-6">
                                    <div class="text-center">
                                        <h3 class="text-lg font-medium text-gray-900 mb-4">
                                            "Sign in with your Google account"
                                        </h3>
                                        <p class="text-sm text-gray-500 mb-6">
                                            "Secure authentication powered by Google OAuth"
                                        </p>
                                    </div>
                                    
                                    <div class="flex justify-center">
                                        <LoginButton class="w-full justify-center py-3 text-base" />
                                    </div>
                                    
                                    <div class="mt-6">
                                        <div class="relative">
                                            <div class="absolute inset-0 flex items-center">
                                                <div class="w-full border-t border-gray-300"></div>
                                            </div>
                                            <div class="relative flex justify-center text-sm">
                                                <span class="px-2 bg-white text-gray-500">
                                                    "Quick & Secure"
                                                </span>
                                            </div>
                                        </div>
                                        
                                        <div class="mt-4 text-center text-xs text-gray-500">
                                            <p>
                                                "By signing in, you agree to our terms of service and privacy policy."
                                            </p>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                        
                        <div class="text-center">
                            <div class="bg-blue-50 border border-blue-200 rounded-lg p-4">
                                <div class="flex">
                                    <div class="flex-shrink-0">
                                        <svg 
                                            class="h-5 w-5 text-blue-400" 
                                            fill="currentColor" 
                                            viewBox="0 0 20 20"
                                        >
                                            <path 
                                                fill-rule="evenodd" 
                                                d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" 
                                                clip-rule="evenodd"
                                            />
                                        </svg>
                                    </div>
                                    <div class="ml-3">
                                        <p class="text-sm text-blue-700">
                                            "New to our platform? " 
                                            <strong>"No account needed!"</strong>
                                            " Just sign in with Google to get started immediately."
                                        </p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
                    }.into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }
        >
            <div class="flex justify-center items-center min-h-screen">
                <div class="flex flex-col items-center space-y-4">
                    <div class="animate-spin rounded-full h-12 w-12 border-4 border-blue-500 border-t-transparent"></div>
                    <p class="text-gray-600">"Loading..."</p>
                </div>
            </div>
        </Show>
    }
}