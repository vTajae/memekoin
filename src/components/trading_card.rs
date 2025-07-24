use leptos::prelude::*;

#[component]
pub fn TradingCard(
    #[prop(into)] title: String,
    #[prop(into)] description: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="bg-white/10 backdrop-blur-xl rounded-2xl border border-white/20 shadow-2xl p-6 hover:bg-white/15 transition-all duration-300 group">
            <div class="flex items-center space-x-3 mb-6">
                <div class="w-10 h-10 bg-gradient-to-r from-blue-500 to-purple-600 rounded-xl flex items-center justify-center text-white font-bold text-lg">
                    {
                        if title.starts_with("📊") { "📊" }
                        else if title.starts_with("💹") { "💹" }
                        else if title.starts_with("📈") { "📈" }
                        else { "💎" }
                    }
                </div>
                <div>
                    <h2 class="text-xl font-bold text-white group-hover:text-yellow-300 transition-colors">
                        {title}
                    </h2>
                    <p class="text-gray-300 text-sm">
                        {description}
                    </p>
                </div>
            </div>
            <div class="space-y-4">
                {children()}
            </div>
        </div>
    }
}
