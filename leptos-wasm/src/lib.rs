use leptos::prelude::*;
use leptos_router::{components::*, path};

// Modules
mod components;
mod pages;
mod services;
mod types;

// Top-Level pages
use crate::pages::home::Home;
use crate::pages::login::LoginPage;
use crate::pages::dashboard::DashboardPage;
use crate::pages::not_found::NotFound;
use crate::pages::oauth_callback::OAuthCallbackPage;
use crate::components::auth::AuthProvider;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
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
