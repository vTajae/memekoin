use leptos::prelude::*;
use leptos_router::components::A;
use crate::auth::{AuthContext, AuthState};

#[component]
pub fn Navbar() -> impl IntoView {
    let auth = expect_context::<AuthContext>();

    let logout_handler = move |_| {
        crate::auth::logout(auth.set_state);
    };

    view! {
        <nav class="bg-white shadow-lg">
            <div class="max-w-7xl mx-auto px-4">
                <div class="flex justify-between h-16">
                    <div class="flex items-center">
                        <A href="/" attr:class="flex-shrink-0 flex items-center">
                            <span class="text-xl font-bold text-gray-800">
                                "Leptos App"
                            </span>
                        </A>
                    </div>
                    
                    <div class="flex items-center space-x-4">
                        <Suspense fallback=move || view! {
                            <div class="animate-pulse">
                                <div class="h-8 w-20 bg-gray-200 rounded"></div>
                            </div>
                        }>
                            {move || match auth.state.get() {
                                AuthState::Loading => view! {
                                    <div class="animate-pulse">
                                        <div class="h-8 w-20 bg-gray-200 rounded"></div>
                                    </div>
                                }.into_any(),
                                AuthState::Authenticated(user) => view! {
                                    <div class="flex items-center space-x-4">
                                        <span class="text-gray-700">
                                            "Welcome, " {user.username}
                                        </span>
                                        <button
                                            on:click=logout_handler
                                            class="bg-red-600 hover:bg-red-700 text-white px-4 py-2 rounded-md text-sm font-medium transition-colors duration-200"
                                        >
                                            "Logout"
                                        </button>
                                    </div>
                                }.into_any(),
                                AuthState::Unauthenticated => view! {
                                    <div class="flex items-center space-x-4">
                                        <A 
                                            href="/login"
                                            attr:class="text-gray-700 hover:text-gray-900 px-3 py-2 rounded-md text-sm font-medium"
                                        >
                                            "Login"
                                        </A>
                                        <A 
                                            href="/register"
                                            attr:class="bg-indigo-600 hover:bg-indigo-700 text-white px-4 py-2 rounded-md text-sm font-medium transition-colors duration-200"
                                        >
                                            "Sign Up"
                                        </A>
                                    </div>
                                }.into_any(),
                            }}
                        </Suspense>
                    </div>
                </div>
            </div>
        </nav>
    }
}

