use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, *};

use crate::pages::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        // Remove duplicate CSS loading - already handled in shell.rs
        <Title text="Meme Koin - Enterprise Crypto Platform"/>
        <Meta name="description" content="Enterprise-grade meme cryptocurrency platform built with Rust and Leptos"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Meta name="theme-color" content="#3b82f6"/>
        // Open Graph meta tags - moved to shell.rs to avoid Leptos preload bug
        // <Meta property="og:title" content="Meme Koin - Enterprise Crypto Platform"/>
        // <Meta property="og:description" content="Enterprise-grade meme cryptocurrency platform built with Rust and Leptos"/>
        // <Meta property="og:type" content="website"/>

        <Router>
            <main class="min-h-screen bg-gray-50">
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=path!("/") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}
