use leptos::prelude::*;

#[component]
pub fn TradingPage() -> impl IntoView {
    // Simple counter for testing
    let (counter, set_counter) = signal(0);

    let increment_counter = move |_| {
        set_counter.update(|count| *count += 1);
    };

    view! {
        <div class="container mx-auto py-12 px-4">
            <div class="text-center mb-12">
                <h1 class="text-5xl font-bold text-gray-900 mb-4">
                    "Trading " <span class="text-blue-600">"Dashboard"</span>
                </h1>
                <p class="text-xl text-gray-600 max-w-2xl mx-auto">
                    "Advanced cryptocurrency trading interface"
                </p>
            </div>

            <div class="max-w-md mx-auto bg-white rounded-lg shadow-md p-6">
                <h2 class="text-2xl font-semibold text-center mb-4">"Counter Demo"</h2>
                <div class="text-center">
                    <div class="text-4xl font-bold text-blue-600 mb-4">
                        {move || counter.get()}
                    </div>
                    <button
                        class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
                        on:click=increment_counter
                    >
                        "Increment Counter"
                    </button>
                </div>
            </div>

            <div class="text-center mt-12">
                <h2 class="text-3xl font-bold text-gray-900 mb-8">"Built with Modern Technology"</h2>
                <div class="grid grid-cols-2 md:grid-cols-4 gap-8 max-w-4xl mx-auto">
                    <div class="text-center">
                        <div class="w-16 h-16 bg-orange-100 rounded-lg flex items-center justify-center mx-auto mb-3">
                            <span class="text-2xl">"🦀"</span>
                        </div>
                        <h3 class="font-semibold text-gray-900">"Rust"</h3>
                        <p class="text-sm text-gray-600">"Memory Safe"</p>
                    </div>
                    <div class="text-center">
                        <div class="w-16 h-16 bg-blue-100 rounded-lg flex items-center justify-center mx-auto mb-3">
                            <span class="text-2xl">"⚡"</span>
                        </div>
                        <h3 class="font-semibold text-gray-900">"Leptos"</h3>
                        <p class="text-sm text-gray-600">"Full-Stack"</p>
                    </div>
                    <div class="text-center">
                        <div class="w-16 h-16 bg-purple-100 rounded-lg flex items-center justify-center mx-auto mb-3">
                            <span class="text-2xl">"☁️"</span>
                        </div>
                        <h3 class="font-semibold text-gray-900">"Cloudflare"</h3>
                        <p class="text-sm text-gray-600">"Edge Computing"</p>
                    </div>
                    <div class="text-center">
                        <div class="w-16 h-16 bg-cyan-100 rounded-lg flex items-center justify-center mx-auto mb-3">
                            <span class="text-2xl">"🎨"</span>
                        </div>
                        <h3 class="font-semibold text-gray-900">"Tailwind"</h3>
                        <p class="text-sm text-gray-600">"Utility-First"</p>
                    </div>
                </div>
            </div>
        </div>
    }
}
