use leptos::prelude::*;
use leptos_router::components::A;
use crate::components::auth::user_profile::UserProfile;
use crate::components::layout::navigation::Navigation;
use crate::context::auth::use_auth_state;
use crate::types::auth::AuthState;

#[component]
pub fn Header() -> impl IntoView {
    let auth_state = use_auth_state();
    let (mobile_menu_open, set_mobile_menu_open) = signal(false);
    
    let close_mobile_menu: Callback<(), ()> = Callback::from(move || {
        set_mobile_menu_open.set(false);
    });

    view! {
        <header class="bg-white shadow-sm border-b border-gray-200">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex justify-between items-center h-16">
                    // Logo and brand
                    <div class="flex items-center">
                        <A href="/">
                            <div class="flex items-center space-x-3">
                                <div class="w-8 h-8 bg-gradient-to-br from-blue-500 to-purple-600 rounded-lg flex items-center justify-center">
                                    <span class="text-white font-bold text-lg">"L"</span>
                                </div>
                                <span class="text-xl font-semibold text-gray-900">
                                    "Leptos App"
                                </span>
                            </div>
                        </A>
                    </div>

                    // Desktop Navigation
                    <Navigation mobile=false />

                    // User profile / Auth actions (desktop)
                    <div class="hidden md:flex items-center space-x-4">
                        <UserProfile show_email=false />
                    </div>

                    // Mobile menu button
                    <div class="md:hidden flex items-center space-x-2">
                        // User profile for mobile
                        <div class="md:hidden">
                            <UserProfile show_email=false />
                        </div>
                        
                        <button
                            type="button"
                            class="text-gray-700 hover:text-gray-900 focus:outline-none focus:text-gray-900 p-2"
                            aria-label="Toggle menu"
                            on:click=move |_| set_mobile_menu_open.update(|open| *open = !*open)
                        >
                            <Show
                                when=move || mobile_menu_open.get()
                                fallback=|| view! {
                                    <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
                                    </svg>
                                }
                            >
                                <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                                </svg>
                            </Show>
                        </button>
                    </div>
                </div>

                // Mobile Navigation Menu
                <Show
                    when=move || mobile_menu_open.get()
                    fallback=|| ()
                >
                    <div class="md:hidden border-t border-gray-200">
                        <div class="pt-2 pb-3 space-y-1">
                            <Navigation mobile=true on_close=close_mobile_menu />
                            
                            // Mobile auth section
                            <div class="border-t border-gray-200 pt-4 pb-3">
                                {move || match auth_state.get() {
                                    AuthState::Authenticated(user) => view! {
                                        <div class="px-4">
                                            <div class="flex items-center">
                                                <div class="flex-shrink-0">
                                                    <div class="h-10 w-10 rounded-full bg-gray-300"></div>
                                                </div>
                                                <div class="ml-3">
                                                    <div class="text-base font-medium text-gray-800">
                                                        {user.name.clone().unwrap_or(user.email.clone())}
                                                    </div>
                                                    <div class="text-sm font-medium text-gray-500">
                                                        {user.email.clone()}
                                                    </div>
                                                </div>
                                            </div>
                                            <div class="mt-3">
                                                <a
                                                    href="/api/auth/logout"
                                                    class="block px-4 py-2 text-base font-medium text-gray-500 hover:text-gray-800 hover:bg-gray-100"
                                                >
                                                    "Sign out"
                                                </a>
                                            </div>
                                        </div>
                                    }.into_any(),
                                    _ => view! {
                                        <div class="px-4">
                                            <A
                                                href="/login"
                                                on:click=move |_| set_mobile_menu_open.set(false)
                                            >
                                                <span class="block w-full text-center px-4 py-2 border border-transparent text-base font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700">
                                                    "Sign in"
                                                </span>
                                            </A>
                                        </div>
                                    }.into_any(),
                                }}
                            </div>
                        </div>
                    </div>
                </Show>
            </div>
        </header>
    }
}