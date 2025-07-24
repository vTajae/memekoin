use leptos::prelude::*;
use leptos_meta::provide_meta_context;
#[cfg(feature = "ssr")]
use leptos_meta::MetaTags;
use crate::components::*;

#[cfg(feature = "ssr")]
pub fn shell(options: leptos::config::LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="stylesheet" href="/pkg/koin.css"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <div class="min-h-screen bg-gradient-to-br from-slate-900 via-purple-900 to-slate-900">
            <Header/>
            <main class="container mx-auto px-4 py-8">
                <TradingDashboard/>
            </main>
            <Footer/>
        </div>
    }
}

#[component]
fn Header() -> impl IntoView {
    view! {
        <header class="bg-black/20 backdrop-blur-xl border-b border-white/10">
            <div class="container mx-auto px-4 py-6">
                <div class="flex items-center justify-between">
                    <div class="flex items-center space-x-4">
                        <div class="w-12 h-12 bg-gradient-to-r from-yellow-400 to-orange-500 rounded-xl flex items-center justify-center text-2xl">
                            "🪙"
                        </div>
                        <div>
                            <h1 class="text-3xl font-bold bg-gradient-to-r from-yellow-400 via-orange-500 to-red-500 bg-clip-text text-transparent">
                                "Meme Koin"
                            </h1>
                            <p class="text-gray-300 text-sm">
                                "High-frequency trading platform"
                            </p>
                        </div>
                    </div>
                    <div class="flex items-center space-x-4">
                        <div class="hidden md:flex items-center space-x-2 bg-green-500/20 px-3 py-1 rounded-full">
                            <div class="w-2 h-2 bg-green-400 rounded-full animate-pulse"></div>
                            <span class="text-green-400 text-sm font-medium">"Live"</span>
                        </div>
                        <div class="text-white/60 text-sm">
                            "Connected"
                        </div>
                    </div>
                </div>
            </div>
        </header>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="bg-black/20 backdrop-blur-xl border-t border-white/10 mt-12">
            <div class="container mx-auto px-4 py-8">
                <div class="flex flex-col md:flex-row items-center justify-between">
                    <div class="flex items-center space-x-2 mb-4 md:mb-0">
                        <span class="text-2xl">"🚀"</span>
                        <div>
                            <h3 class="text-white font-semibold">
                                "Powered by Leptos + Cloudflare Workers"
                            </h3>
                            <p class="text-gray-400 text-sm">
                                "Built for maximum performance and scalability"
                            </p>
                        </div>
                    </div>
                    <div class="flex items-center space-x-6 text-sm text-gray-400">
                        <span>"Real-time data"</span>
                        <span>"•"</span>
                        <span>"Secure trading"</span>
                        <span>"•"</span>
                        <span>"24/7 uptime"</span>
                    </div>
                </div>
            </div>
        </footer>
    }
}
