use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

// Modules
mod auth;
mod components;
mod pages;

// Top-Level pages
use crate::pages::home::Home;
use crate::pages::not_found::NotFound;
use crate::components::navbar::Navbar;
use crate::components::login::{LoginForm, RegisterForm};
use crate::auth::{provide_auth_context, AuthContext, AuthState};

/// Dashboard page for authenticated users
#[component]
fn Dashboard() -> impl IntoView {
    view! {
        <div class="container mx-auto px-4 py-8">
            <h1 class="text-3xl font-bold text-gray-900 mb-6">"Dashboard"</h1>
            <div class="bg-white shadow-lg rounded-lg p-6">
                <p class="text-gray-700">"Welcome to your dashboard! This is a protected page that requires authentication."</p>
            </div>
        </div>
    }
}

/// Protected Dashboard that checks authentication
#[component]
fn ProtectedDashboard() -> impl IntoView {
    let auth = expect_context::<AuthContext>();

    view! {
        <Show
            when=move || matches!(auth.state.get(), AuthState::Authenticated(_))
            fallback=move || view! {
                <Show
                    when=move || matches!(auth.state.get(), AuthState::Loading)
                    fallback=move || view! {
                        <div class="min-h-screen flex items-center justify-center bg-gray-50">
                            <div class="max-w-md w-full space-y-8 text-center">
                                <div>
                                    <h2 class="mt-6 text-3xl font-extrabold text-gray-900">
                                        "Access Denied"
                                    </h2>
                                    <p class="mt-2 text-sm text-gray-600">
                                        "You need to be logged in to access this page."
                                    </p>
                                </div>
                                <div class="space-y-4">
                                    <A 
                                        href="/login"
                                        attr:class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                                    >
                                        "Sign In"
                                    </A>
                                    <A 
                                        href="/register"
                                        attr:class="w-full flex justify-center py-2 px-4 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                                    >
                                        "Create Account"
                                    </A>
                                </div>
                            </div>
                        </div>
                    }
                >
                    <div class="min-h-screen flex items-center justify-center">
                        <div class="animate-spin rounded-full h-32 w-32 border-b-2 border-indigo-600"></div>
                    </div>
                </Show>
            }
        >
            <Dashboard />
        </Show>
    }
}

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    
    // Provides authentication context
    provide_auth_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />

        // sets the document title
        <Title text="Welcome to Leptos CSR" />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        // Body needs to be here instead of the index.html for tailwindcss to work properly.
        // Example for a spa:
        <body class="flex flex-col h-screen bg-gray-50">
            <Router>
                <Navbar />
                <main class="flex-grow">
                    <Routes fallback=|| view! { <NotFound /> }>
                        <Route path=path!("/") view=Home />
                        <Route path=path!("/login") view=LoginForm />
                        <Route path=path!("/register") view=RegisterForm />
                        <Route path=path!("/dashboard") view=ProtectedDashboard />
                    </Routes>
                </main>
            </Router>
        </body>
    }
}
