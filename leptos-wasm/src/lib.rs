use leptos::prelude::*;
use tracing::info;

use leptos_router::{components::*, path};

// Modules
mod components;
mod pages;
mod services;
mod types;
mod context;

// Top-Level pages
use crate::pages::home::Home;
use crate::pages::login::LoginPage;
use crate::pages::dashboard::DashboardPage;
use crate::pages::not_found::NotFound;
use crate::pages::oauth_callback::OAuthCallbackPage;
fn init_tracing() {
    // Send tracing logs to browser console with pretty formatting
    tracing_wasm::set_as_global_default();
    console_error_panic_hook::set_once();
    info!("Tracing initialized");
}

use crate::context::auth::AuthProvider;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    init_tracing();
    view! {
        <div class="min-h-screen bg-gray-50">
            <Router>
                <AuthProvider>
                    <Routes fallback=NotFound>
                        <Route path=path!("/") view=Home />
                        <Route path=path!("/login") view=LoginPage />
                        <Route path=path!("/dashboard") view=DashboardPage />
                        <Route path=path!("/auth/callback") view=OAuthCallbackPage />
                    </Routes>
                </AuthProvider>
            </Router>
        </div>
    }
}
