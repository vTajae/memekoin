use leptos::prelude::*;
use leptos_router::components::Redirect;
use crate::components::layout::protected_route::use_protected_route;
use crate::components::auth::user_profile::{UserProfile, UserName, UserAvatar};
use crate::components::auth::use_current_user;

#[component]
pub fn DashboardPage() -> impl IntoView {
    let current_user = use_current_user();
    let auth_guard = use_protected_route();

    view! {
        <Show
            when=move || {
                let (loading, _redirect, _authenticated) = auth_guard.get();
                loading
            }
            fallback=move || {
                let (_loading, redirect, authenticated) = auth_guard.get();
                if redirect {
                    view! { <Redirect path="/login" /> }.into_any()
                } else if authenticated {
                    view! {
                        <div class="min-h-screen bg-gray-50">
                // Header
                <header class="bg-white shadow">
                    <div class="max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8">
                        <div class="flex justify-between items-center">
                            <div class="flex items-center space-x-4">
                                <h1 class="text-3xl font-bold text-gray-900">
                                    "Dashboard"
                                </h1>
                                <div class="hidden sm:block">
                                    {move || match current_user.get() {
                                        Some(user) => view! {
                                            <span class="text-lg text-gray-600">
                                                "Welcome back, " 
                                                <span class="font-medium">
                                                    {user.name.as_ref().unwrap_or(&user.email).clone()}
                                                </span>
                                                "!"
                                            </span>
                                        }.into_any(),
                                        None => view! { <span></span> }.into_any(),
                                    }}
                                </div>
                            </div>
                            <UserProfile show_email=true />
                        </div>
                    </div>
                </header>

                // Main content
                <main>
                    <div class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
                        <div class="px-4 py-6 sm:px-0">
                            // Welcome section
                            <div class="mb-8">
                                <div class="bg-gradient-to-r from-blue-500 to-purple-600 rounded-lg shadow-lg">
                                    <div class="px-6 py-8 text-white">
                                        <div class="flex items-center space-x-4">
                                            <UserAvatar size=64 class="border-2 border-white" />
                                            <div>
                                                <h2 class="text-2xl font-bold">
                                                    "Welcome to your Dashboard"
                                                </h2>
                                                <p class="text-blue-100 mt-1">
                                                    "Manage your account and explore our features"
                                                </p>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>

                            // Dashboard grid
                            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 mb-8">
                                // User info card
                                <div class="bg-white overflow-hidden shadow rounded-lg">
                                    <div class="p-6">
                                        <div class="flex items-center">
                                            <div class="flex-shrink-0">
                                                <svg 
                                                    class="h-8 w-8 text-blue-600" 
                                                    fill="none" 
                                                    stroke="currentColor" 
                                                    viewBox="0 0 24 24"
                                                >
                                                    <path 
                                                        stroke-linecap="round" 
                                                        stroke-linejoin="round" 
                                                        stroke-width="2" 
                                                        d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
                                                    />
                                                </svg>
                                            </div>
                                            <div class="ml-5 w-0 flex-1">
                                                <dl>
                                                    <dt class="text-sm font-medium text-gray-500 truncate">
                                                        "Account Information"
                                                    </dt>
                                                    <dd class="text-lg font-medium text-gray-900">
                                                        <UserName fallback_to_email=true />
                                                    </dd>
                                                </dl>
                                            </div>
                                        </div>
                                        <div class="mt-4">
                                            {move || match current_user.get() {
                                                Some(user) => view! {
                                                    <div class="text-sm text-gray-600">
                                                        <p>"Email: " {user.email.clone()}</p>
                                                        <p class="mt-1">"User ID: " {user.id.clone()}</p>
                                                    </div>
                                                }.into_any(),
                                                None => view! { <div></div> }.into_any(),
                                            }}
                                        </div>
                                    </div>
                                </div>

                                // Quick actions card
                                <div class="bg-white overflow-hidden shadow rounded-lg">
                                    <div class="p-6">
                                        <div class="flex items-center">
                                            <div class="flex-shrink-0">
                                                <svg 
                                                    class="h-8 w-8 text-green-600" 
                                                    fill="none" 
                                                    stroke="currentColor" 
                                                    viewBox="0 0 24 24"
                                                >
                                                    <path 
                                                        stroke-linecap="round" 
                                                        stroke-linejoin="round" 
                                                        stroke-width="2" 
                                                        d="M9 5H7a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"
                                                    />
                                                </svg>
                                            </div>
                                            <div class="ml-5 w-0 flex-1">
                                                <dl>
                                                    <dt class="text-sm font-medium text-gray-500 truncate">
                                                        "Quick Actions"
                                                    </dt>
                                                    <dd class="text-lg font-medium text-gray-900">
                                                        "Get Started"
                                                    </dd>
                                                </dl>
                                            </div>
                                        </div>
                                        <div class="mt-4">
                                            <div class="space-y-2">
                                                <button class="w-full text-left px-3 py-2 text-sm text-blue-600 hover:bg-blue-50 rounded-md transition-colors">
                                                    "View Profile"
                                                </button>
                                                <button class="w-full text-left px-3 py-2 text-sm text-blue-600 hover:bg-blue-50 rounded-md transition-colors">
                                                    "Account Settings"
                                                </button>
                                                <button class="w-full text-left px-3 py-2 text-sm text-blue-600 hover:bg-blue-50 rounded-md transition-colors">
                                                    "Help & Support"
                                                </button>
                                            </div>
                                        </div>
                                    </div>
                                </div>

                                // Stats card
                                <div class="bg-white overflow-hidden shadow rounded-lg">
                                    <div class="p-6">
                                        <div class="flex items-center">
                                            <div class="flex-shrink-0">
                                                <svg 
                                                    class="h-8 w-8 text-purple-600" 
                                                    fill="none" 
                                                    stroke="currentColor" 
                                                    viewBox="0 0 24 24"
                                                >
                                                    <path 
                                                        stroke-linecap="round" 
                                                        stroke-linejoin="round" 
                                                        stroke-width="2" 
                                                        d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
                                                    />
                                                </svg>
                                            </div>
                                            <div class="ml-5 w-0 flex-1">
                                                <dl>
                                                    <dt class="text-sm font-medium text-gray-500 truncate">
                                                        "Your Activity"
                                                    </dt>
                                                    <dd class="text-lg font-medium text-gray-900">
                                                        "Welcome!"
                                                    </dd>
                                                </dl>
                                            </div>
                                        </div>
                                        <div class="mt-4 text-sm text-gray-600">
                                            <p>"You're successfully authenticated and ready to explore."</p>
                                        </div>
                                    </div>
                                </div>
                            </div>

                            // Main content area
                            <div class="bg-white shadow rounded-lg">
                                <div class="px-6 py-8">
                                    <div class="text-center">
                                        <svg 
                                            class="mx-auto h-16 w-16 text-gray-400" 
                                            fill="none" 
                                            stroke="currentColor" 
                                            viewBox="0 0 24 24"
                                        >
                                            <path 
                                                stroke-linecap="round" 
                                                stroke-linejoin="round" 
                                                stroke-width="2" 
                                                d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"
                                            />
                                        </svg>
                                        <h3 class="mt-4 text-lg font-medium text-gray-900">
                                            "Dashboard Content"
                                        </h3>
                                        <p class="mt-2 text-sm text-gray-500 max-w-sm mx-auto">
                                            "This is your main dashboard area. Future features and content will be added here as your application grows."
                                        </p>
                                        <div class="mt-6">
                                            <div class="inline-flex rounded-md shadow">
                                                <button 
                                                    type="button" 
                                                    class="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                                                >
                                                    "Explore Features"
                                                </button>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </main>
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