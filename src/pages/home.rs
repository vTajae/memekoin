use leptos::prelude::*;
use leptos::ev;
use leptos::task::spawn_local;

use crate::components::ui::*;
use crate::server::functions::*;

#[component]
pub fn HomePage() -> impl IntoView {
    // Use lazy initialization to reduce initial bundle size
    let counter = RwSignal::new(0);
    let api_response = RwSignal::new(String::new());
    let is_loading = RwSignal::new(false);

    let increment_counter = Callback::new(move |_: ev::MouseEvent| {
        counter.update(|count| *count += 1);
    });

    let call_api = Callback::new(move |_: ev::MouseEvent| {
        if is_loading.get() {
            return; // Prevent multiple simultaneous calls
        }

        is_loading.set(true);
        spawn_local(async move {
            match say_hello(counter.get()).await {
                Ok(response) => {
                    api_response.set(response);
                    is_loading.set(false);
                },
                Err(e) => {
                    api_response.set(format!("Error: {}", e));
                    is_loading.set(false);
                },
            }
        });
    });

    view! {
        <div class="container py-12">
            <div class="text-center mb-12">
                <h1 class="text-5xl font-bold text-gray-900 mb-4">
                    "Welcome to " <span class="text-primary-600">"Meme Koin"</span>
                </h1>
                <p class="text-xl text-gray-600 max-w-2xl mx-auto">
                    "Enterprise-grade meme cryptocurrency platform built with Rust, Leptos, and Cloudflare Workers"
                </p>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8 mb-12">
                <Card>
                    <CardHeader>
                        <h2 class="text-xl font-semibold">"Counter Demo"</h2>
                    </CardHeader>
                    <CardContent>
                        <div class="text-center">
                            <div class="text-4xl font-bold text-primary-600 mb-4">
                                {move || counter.get()}
                            </div>
                            <Button
                                variant=ButtonVariant::Primary
                                size=ButtonSize::Medium
                                on_click=increment_counter
                            >
                                "Increment Counter"
                            </Button>
                        </div>
                    </CardContent>
                </Card>

                <Card>
                    <CardHeader>
                        <h2 class="text-xl font-semibold">"Server Function Demo"</h2>
                    </CardHeader>
                    <CardContent>
                        <div class="text-center">
                            <div class="mb-4 min-h-[3rem] flex items-center justify-center">
                                <p class="text-sm text-gray-600">
                                    {move || {
                                        let response = api_response.get();
                                        if response.is_empty() {
                                            "Click to call server function".to_string()
                                        } else {
                                            response
                                        }
                                    }}
                                </p>
                            </div>
                            <Button
                                variant=ButtonVariant::Secondary
                                size=ButtonSize::Medium
                                on_click=call_api
                            >
                                "Call API"
                            </Button>
                        </div>
                    </CardContent>
                </Card>

                <Card>
                    <CardHeader>
                        <h2 class="text-xl font-semibold">"Enterprise Features"</h2>
                    </CardHeader>
                    <CardContent>
                        <ul class="space-y-2 text-sm text-gray-600">
                            <li class="flex items-center">
                                <span class="w-2 h-2 bg-green-500 rounded-full mr-2"></span>
                                "Modular Architecture"
                            </li>
                            <li class="flex items-center">
                                <span class="w-2 h-2 bg-green-500 rounded-full mr-2"></span>
                                "Type-Safe Server Functions"
                            </li>
                            <li class="flex items-center">
                                <span class="w-2 h-2 bg-green-500 rounded-full mr-2"></span>
                                "Tailwind CSS Integration"
                            </li>
                            <li class="flex items-center">
                                <span class="w-2 h-2 bg-green-500 rounded-full mr-2"></span>
                                "Cloudflare Workers"
                            </li>
                        </ul>
                    </CardContent>
                </Card>
            </div>

            <div class="text-center">
                <h2 class="text-3xl font-bold text-gray-900 mb-8">"Built with Modern Technology"</h2>
                <div class="grid grid-cols-2 md:grid-cols-4 gap-8">
                    <div class="text-center">
                        <div class="w-16 h-16 bg-orange-100 rounded-lg flex items-center justify-center mx-auto mb-3">
                            <span class="text-2xl font-bold text-orange-600">"🦀"</span>
                        </div>
                        <h3 class="font-semibold text-gray-900">"Rust"</h3>
                        <p class="text-sm text-gray-600">"Memory Safe"</p>
                    </div>
                    <div class="text-center">
                        <div class="w-16 h-16 bg-blue-100 rounded-lg flex items-center justify-center mx-auto mb-3">
                            <span class="text-2xl font-bold text-blue-600">"⚡"</span>
                        </div>
                        <h3 class="font-semibold text-gray-900">"Leptos"</h3>
                        <p class="text-sm text-gray-600">"Full-Stack"</p>
                    </div>
                    <div class="text-center">
                        <div class="w-16 h-16 bg-purple-100 rounded-lg flex items-center justify-center mx-auto mb-3">
                            <span class="text-2xl font-bold text-purple-600">"☁️"</span>
                        </div>
                        <h3 class="font-semibold text-gray-900">"Cloudflare"</h3>
                        <p class="text-sm text-gray-600">"Edge Computing"</p>
                    </div>
                    <div class="text-center">
                        <div class="w-16 h-16 bg-cyan-100 rounded-lg flex items-center justify-center mx-auto mb-3">
                            <span class="text-2xl font-bold text-cyan-600">"🎨"</span>
                        </div>
                        <h3 class="font-semibold text-gray-900">"Tailwind"</h3>
                        <p class="text-sm text-gray-600">"Utility-First"</p>
                    </div>
                </div>
            </div>
        </div>
    }
}
