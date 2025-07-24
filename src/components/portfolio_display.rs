use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioItem {
    pub symbol: String,
    pub amount: f64,
    pub value: f64,
}

#[component]
pub fn PortfolioDisplay() -> impl IntoView {
    let portfolio_data = RwSignal::new(vec![
        PortfolioItem { symbol: "BTC".to_string(), amount: 0.25, value: 11250.0 },
        PortfolioItem { symbol: "ETH".to_string(), amount: 0.39, value: 1248.0 },
        PortfolioItem { symbol: "DOGE".to_string(), amount: 25.0, value: 2.0 },
    ]);

    let total_value = move || {
        portfolio_data.get().iter().map(|item| item.value).sum::<f64>()
    };

    view! {
        <div id="portfolio-data" class="space-y-6">
            <div class="bg-gradient-to-r from-blue-600 to-purple-600 rounded-2xl p-6 text-white">
                <div class="flex items-center justify-between">
                    <div>
                        <div class="text-3xl font-bold">${total_value()}</div>
                        <div class="text-blue-100 text-sm">Total Portfolio Value</div>
                    </div>
                    <div class="w-16 h-16 bg-white/20 rounded-full flex items-center justify-center">
                        <span class="text-2xl">"💰"</span>
                    </div>
                </div>
                <div class="mt-4 flex items-center space-x-4 text-sm">
                    <div class="flex items-center space-x-1">
                        <div class="w-2 h-2 bg-green-400 rounded-full"></div>
                        <span class="text-blue-100">"+12.5% today"</span>
                    </div>
                    <div class="flex items-center space-x-1">
                        <div class="w-2 h-2 bg-yellow-400 rounded-full"></div>
                        <span class="text-blue-100">"3 assets"</span>
                    </div>
                </div>
            </div>

            <div class="space-y-4">
                <For
                    each=move || portfolio_data.get()
                    key=|item| item.symbol.clone()
                    children=move |item| {
                        let symbol_icon = match item.symbol.as_str() {
                            "BTC" => "🟠",
                            "ETH" => "🔷",
                            "DOGE" => "🐕",
                            _ => "💎"
                        };

                        view! {
                            <div class="bg-white/5 rounded-xl p-4 border border-white/10 hover:bg-white/10 transition-all duration-200 group">
                                <div class="flex items-center justify-between">
                                    <div class="flex items-center space-x-4">
                                        <div class="w-12 h-12 bg-gradient-to-r from-gray-600 to-gray-800 rounded-xl flex items-center justify-center text-xl">
                                            {symbol_icon}
                                        </div>
                                        <div>
                                            <div class="font-bold text-white text-lg">{item.symbol}</div>
                                            <div class="text-gray-400 text-sm">{item.amount}" coins"</div>
                                        </div>
                                    </div>
                                    <div class="text-right">
                                        <div class="font-bold text-white text-xl">${item.value}</div>
                                        <div class="text-green-400 text-sm font-medium">"+5.2%"</div>
                                    </div>
                                </div>
                                <div class="mt-3 bg-white/5 rounded-lg h-2 overflow-hidden">
                                    <div class="bg-gradient-to-r from-blue-500 to-purple-600 h-full" style="width: 75%"></div>
                                </div>
                            </div>
                        }
                    }
                />
            </div>

            <div class="bg-yellow-500/10 border border-yellow-500/20 rounded-xl p-4">
                <div class="flex items-center space-x-2 text-yellow-400">
                    <span class="text-lg">"⚡"</span>
                    <span class="font-medium">"Portfolio Insights"</span>
                </div>
                <p class="text-gray-300 text-sm mt-2">
                    "Your portfolio is well-diversified across major cryptocurrencies. Consider rebalancing if any single asset exceeds 60% allocation."
                </p>
            </div>
        </div>
    }
}
