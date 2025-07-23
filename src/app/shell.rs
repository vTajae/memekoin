use leptos::prelude::*;
use leptos_meta::*;
use crate::app::App;

#[component]
pub fn Shell(_options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en" class="h-full">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>

                // Early script to suppress WASM preload warnings before they can be generated
                <script>
                    {r#"
                    (function() {
                        // Override console.warn early to filter out preload warnings
                        const originalWarn = console.warn;
                        console.warn = function(...args) {
                            const message = String(args[0] || '');
                            // Suppress specific WASM preload warnings
                            if (message.includes('preload') &&
                                (message.includes('not used because the request credentials mode') ||
                                 message.includes('not used within a few seconds'))) {
                                return; // Suppress these warnings
                            }
                            originalWarn.apply(console, args);
                        };
                    })();
                    "#}
                </script>

                // Preload critical resources in order of importance with proper crossorigin
                // Add WASM preload with correct crossorigin to override Leptos auto-generation
                <link rel="preload" href="/pkg/koin.wasm" r#as="fetch" r#type="application/wasm" crossorigin="anonymous" importance="high"/>
                <link rel="preload" href="/pkg/koin.css" r#as="style" importance="high"/>
                <link rel="preload" href="/pkg/koin.js" r#as="script" importance="high" crossorigin="anonymous"/>





                // DNS prefetch for external resources
                <link rel="dns-prefetch" href="https://fonts.googleapis.com"/>
                <link rel="dns-prefetch" href="https://fonts.gstatic.com"/>

                // Prefetch non-critical resources for better caching
                <link rel="prefetch" href="/sw.js"/>
                <link rel="prefetch" href="/favicon.ico"/>

                // Note: HydrationScripts removed because it auto-generates WASM preloads without crossorigin
                // Leptos will still auto-generate the necessary hydration script in the body

                // Optimized favicon - prevent 404 errors
                <link rel="icon" href="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><text y='.9em' font-size='90'>🪙</text></svg>"/>
                <link rel="apple-touch-icon" href="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><text y='.9em' font-size='90'>🪙</text></svg>"/>
                <link rel="shortcut icon" href="data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><text y='.9em' font-size='90'>🪙</text></svg>"/>

                <MetaTags/>

                // Open Graph meta tags (moved from router.rs to avoid Leptos preload bug)
                <meta property="og:title" content="Meme Koin - Enterprise Crypto Platform"/>
                <meta property="og:description" content="Enterprise-grade meme cryptocurrency platform built with Rust and Leptos"/>
                <meta property="og:type" content="website"/>

                // Critical CSS inlined for immediate rendering
                <style>
                    {r#"
                    /* Critical above-the-fold styles with optimized font stack */
                    html, body {
                        font-family: 'Inter-fallback', 'Inter', system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
                        background-color: #f9fafb;
                        margin: 0;
                        padding: 0;
                        line-height: 1.6;
                        color: #111827;
                        font-display: swap;
                        -webkit-font-smoothing: antialiased;
                        -moz-osx-font-smoothing: grayscale;
                        text-rendering: optimizeLegibility;
                    }
                    .container {
                        max-width: 1200px;
                        margin: 0 auto;
                        padding: 0 1rem;
                    }
                    .text-center { text-align: center; }
                    .mb-4 { margin-bottom: 1rem; }
                    .mb-8 { margin-bottom: 2rem; }
                    .mb-12 { margin-bottom: 3rem; }
                    .py-12 { padding-top: 3rem; padding-bottom: 3rem; }
                    .text-5xl { font-size: 3rem; line-height: 1; }
                    .text-xl { font-size: 1.25rem; line-height: 1.75rem; }
                    .font-bold { font-weight: 700; }
                    .text-gray-900 { color: #111827; }
                    .text-gray-600 { color: #4b5563; }
                    .text-primary-600 { color: #2563eb; }
                    .min-h-screen { min-height: 100vh; }
                    .bg-gray-50 { background-color: #f9fafb; }
                    .antialiased { -webkit-font-smoothing: antialiased; -moz-osx-font-smoothing: grayscale; }
                    .h-full { height: 100%; }
                    .max-w-2xl { max-width: 42rem; }
                    .mx-auto { margin-left: auto; margin-right: auto; }

                    /* Loading state to prevent FOUC */
                    .loading { opacity: 0; }
                    .loaded { opacity: 1; transition: opacity 0.3s ease-in-out; }

                    /* Initial loading screen for better perceived performance */
                    .initial-loader {
                        position: fixed;
                        top: 0;
                        left: 0;
                        width: 100%;
                        height: 100%;
                        background: #f9fafb;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        z-index: 9999;
                        transition: opacity 0.3s ease-out;
                    }

                    .initial-loader.hidden {
                        opacity: 0;
                        pointer-events: none;
                    }

                    .loader-content {
                        text-align: center;
                        max-width: 400px;
                        padding: 2rem;
                    }

                    .loader-title {
                        font-size: 2.5rem;
                        font-weight: 700;
                        color: #111827;
                        margin-bottom: 0.5rem;
                    }

                    .loader-subtitle {
                        color: #6b7280;
                        margin-bottom: 2rem;
                        font-size: 1rem;
                    }

                    .progress-bar {
                        width: 100%;
                        height: 4px;
                        background: #e5e7eb;
                        border-radius: 2px;
                        overflow: hidden;
                        margin-bottom: 1rem;
                    }

                    .progress-fill {
                        height: 100%;
                        background: linear-gradient(90deg, #3b82f6, #1d4ed8);
                        border-radius: 2px;
                        animation: progress 2s ease-in-out infinite;
                    }

                    @keyframes progress {
                        0% { width: 0%; }
                        50% { width: 70%; }
                        100% { width: 100%; }
                    }

                    /* Skeleton loading animation */
                    .skeleton {
                        background: linear-gradient(90deg, #f0f0f0 25%, #e0e0e0 50%, #f0f0f0 75%);
                        background-size: 200% 100%;
                        animation: skeleton-loading 1.5s infinite;
                        border-radius: 0.375rem;
                    }

                    @keyframes skeleton-loading {
                        0% { background-position: 200% 0; }
                        100% { background-position: -200% 0; }
                    }
                    "#}
                </style>

                // Load main CSS with media attribute to prevent render blocking
                <link rel="stylesheet" href="/pkg/koin.css" media="print" onload="this.media='all'; this.onload=null;"/>
                <noscript><link rel="stylesheet" href="/pkg/koin.css"/></noscript>

                // Optimized font loading strategy - system fonts first, web fonts as enhancement
                <style>
                    {r#"
                    /* Local font fallbacks for better performance */
                    @font-face {
                        font-family: 'Inter-fallback';
                        src: local('Inter'), local('Inter-Regular');
                        font-weight: 400;
                        font-style: normal;
                        font-display: swap;
                    }
                    @font-face {
                        font-family: 'Inter-fallback';
                        src: local('Inter Medium'), local('Inter-Medium');
                        font-weight: 500;
                        font-style: normal;
                        font-display: swap;
                    }
                    @font-face {
                        font-family: 'Inter-fallback';
                        src: local('Inter Bold'), local('Inter-Bold');
                        font-weight: 700;
                        font-style: normal;
                        font-display: swap;
                    }
                    "#}
                </style>

                // Preconnect for faster font loading
                <link rel="preconnect" href="https://fonts.googleapis.com"/>
                <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous"/>

                // Load web fonts asynchronously as progressive enhancement
                <link rel="preload" href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&display=swap" r#as="style" onload="this.onload=null;this.rel='stylesheet'"/>
                <noscript><link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&display=swap" rel="stylesheet"/></noscript>

                // Note: modulepreload already handled in head with proper crossorigin

                // Enhanced performance and loading management script
                <script>
                    {r#"
                    // Performance optimization and loading management
                    (function() {
                        let wasmLoaded = false;
                        let domReady = false;

                        // Hide loader when both WASM and DOM are ready
                        function checkAndHideLoader() {
                            if (wasmLoaded && domReady) {
                                const loader = document.getElementById('initial-loader');
                                if (loader) {
                                    loader.classList.add('hidden');
                                    setTimeout(() => loader.remove(), 300);
                                }
                                document.body.classList.remove('loading');
                                document.body.classList.add('loaded');
                            }
                        }

                        // DOM ready handler
                        document.addEventListener('DOMContentLoaded', function() {
                            domReady = true;
                            checkAndHideLoader();

                            // Fallback: hide loader after maximum wait time
                            setTimeout(function() {
                                const loader = document.getElementById('initial-loader');
                                if (loader && !loader.classList.contains('hidden')) {
                                    loader.classList.add('hidden');
                                    setTimeout(() => loader.remove(), 300);
                                }
                                document.body.classList.remove('loading');
                                document.body.classList.add('loaded');
                            }, 8000);
                        });

                        // Listen for WASM initialization
                        window.addEventListener('wasmLoaded', function() {
                            wasmLoaded = true;
                            checkAndHideLoader();
                        });

                        // Register service worker for caching
                        if ('serviceWorker' in navigator) {
                            window.addEventListener('load', function() {
                                navigator.serviceWorker.register('/sw.js').catch(function(error) {
                                    console.log('ServiceWorker registration failed: ', error);
                                });
                            });
                        }

                        // Listen for WASM initialization completion
                        window.addEventListener('wasmLoaded', function() {
                            wasmLoaded = true;
                            checkAndHideLoader();
                        });
                    })();
                    "#}
                </script>
            </head>
            <body class="h-full bg-gray-50 antialiased loading">
                // Initial loading screen for better perceived performance
                <div id="initial-loader" class="initial-loader">
                    <div class="loader-content">
                        <div class="loader-title">"🪙 Meme Koin"</div>
                        <div class="loader-subtitle">Loading enterprise crypto platform...</div>
                        <div class="progress-bar">
                            <div class="progress-fill"></div>
                        </div>
                        <div style="font-size: 0.875rem; color: #9ca3af;">Initializing WASM runtime</div>
                    </div>
                </div>

                <App/>

                // Suppress WASM warnings by overriding console methods before any scripts run
                <script>
                    {r#"
                    (function() {
                        // Store original console methods
                        const originalWarn = console.warn;
                        const originalLog = console.log;
                        const originalError = console.error;

                        // Override console.warn to filter out WASM deprecation warnings
                        console.warn = function(...args) {
                            const message = String(args[0] || '');
                            if (message.includes('using deprecated parameters for the initialization function') ||
                                message.includes('pass a single object instead')) {
                                return; // Suppress this warning
                            }
                            originalWarn.apply(console, args);
                        };

                        // Override console.log to filter out preload warnings
                        console.log = function(...args) {
                            const message = String(args[0] || '');
                            if (message.includes('preload') && message.includes('not used because the request credentials mode')) {
                                return; // Suppress this warning
                            }
                            originalLog.apply(console, args);
                        };

                        // Also check console.error for any WASM-related errors
                        console.error = function(...args) {
                            const message = String(args[0] || '');
                            if (message.includes('preload') && message.includes('not used within a few seconds')) {
                                return; // Suppress this warning
                            }
                            originalError.apply(console, args);
                        };
                    })();
                    "#}
                </script>
            </body>
        </html>
    }
}
