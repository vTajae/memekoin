use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_location;
use crate::context::auth::use_auth_state;
use crate::types::auth::AuthState;

/// Navigation component with client-side routing
#[component]
pub fn Navigation(
    /// Whether to show mobile menu
    #[prop(optional)]
    mobile: bool,
    /// Callback to close mobile menu
    #[prop(optional)]
    on_close: Option<Callback<(), ()>>,
) -> impl IntoView {
    let auth_state = use_auth_state();
    let location = use_location();
    
    let nav_class = if mobile {
        "flex flex-col space-y-4"
    } else {
        "hidden md:flex items-center space-x-8"
    };
    
    let handle_click = move |_| {
        if let Some(on_close) = on_close.clone() {
            on_close.run(());
        }
    };

    view! {
        <nav class=nav_class>
            <A 
                href="/"
                on:click=handle_click
            >
                <span class=move || {
                    let path = location.pathname.get();
                    let base_class = if mobile {
                        "block px-3 py-2 rounded-md text-base font-medium"
                    } else {
                        ""
                    };
                    
                    if path == "/" {
                        format!("{} text-blue-600 font-medium", base_class)
                    } else {
                        format!("{} text-gray-700 hover:text-blue-600 hover:bg-gray-50 transition-colors", base_class)
                    }
                }>
                    "Home"
                </span>
            </A>
            
            {move || match auth_state.get() {
                AuthState::Authenticated(_) => view! {
                    <>
                        <A 
                            href="/dashboard"
                            on:click=handle_click
                        >
                            <span class=move || {
                                let path = location.pathname.get();
                                let base_class = if mobile {
                                    "block px-3 py-2 rounded-md text-base font-medium"
                                } else {
                                    ""
                                };
                                
                                if path == "/dashboard" {
                                    format!("{} text-blue-600 font-medium", base_class)
                                } else {
                                    format!("{} text-gray-700 hover:text-blue-600 hover:bg-gray-50 transition-colors", base_class)
                                }
                            }>
                                "Dashboard"
                            </span>
                        </A>
                        
                        <A 
                            href="/profile"
                            on:click=handle_click
                        >
                            <span class=move || {
                                let path = location.pathname.get();
                                let base_class = if mobile {
                                    "block px-3 py-2 rounded-md text-base font-medium"
                                } else {
                                    ""
                                };
                                
                                if path == "/profile" {
                                    format!("{} text-blue-600 font-medium", base_class)
                                } else {
                                    format!("{} text-gray-700 hover:text-blue-600 hover:bg-gray-50 transition-colors", base_class)
                                }
                            }>
                                "Profile"
                            </span>
                        </A>
                    </>
                }.into_any(),
                _ => view! {
                    <>
                        <A 
                            href="/login"
                            on:click=handle_click
                        >
                            <span class=move || {
                                let path = location.pathname.get();
                                let base_class = if mobile {
                                    "block px-3 py-2 rounded-md text-base font-medium"
                                } else {
                                    ""
                                };
                                
                                if path == "/login" {
                                    format!("{} text-blue-600 font-medium", base_class)
                                } else {
                                    format!("{} text-gray-700 hover:text-blue-600 hover:bg-gray-50 transition-colors", base_class)
                                }
                            }>
                                "Login"
                            </span>
                        </A>
                        
                        <A 
                            href="/about"
                            on:click=handle_click
                        >
                            <span class=move || {
                                let path = location.pathname.get();
                                let base_class = if mobile {
                                    "block px-3 py-2 rounded-md text-base font-medium"
                                } else {
                                    ""
                                };
                                
                                if path == "/about" {
                                    format!("{} text-blue-600 font-medium", base_class)
                                } else {
                                    format!("{} text-gray-700 hover:text-blue-600 hover:bg-gray-50 transition-colors", base_class)
                                }
                            }>
                                "About"
                            </span>
                        </A>
                    </>
                }.into_any(),
            }}
        </nav>
    }
}