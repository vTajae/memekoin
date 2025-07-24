use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketData {
    pub symbol: String,
    pub price: f64,
    pub change: f64,
}

#[component]
pub fn MarketOverview() -> impl IntoView {
    let market_data = RwSignal::new(vec![
        MarketData { symbol: "BTC".to_string(), price: 45000.0, change: 2.5 },
        MarketData { symbol: "ETH".to_string(), price: 3200.0, change: -1.2 },
        MarketData { symbol: "DOGE".to_string(), price: 0.08, change: 15.7 },
    ]);

    view! {
        <div id="market-data" class="space-y-3">
            <For
                each=move || market_data.get()
                key=|item| item.symbol.clone()
                children=move |item| {
                    let (change_class, icon) = if item.change >= 0.0 {
                        ("text-green-400", "↗")
                    } else {
                        ("text-red-400", "↘")
                    };
                    view! {
                        <div class="bg-white/5 rounded-xl p-4 border border-white/10 hover:bg-white/10 transition-all duration-200 group">
                            <div class="flex items-center justify-between">
                                <div class="flex items-center space-x-3">
                                    <div class="w-10 h-10 bg-gradient-to-r from-orange-400 to-yellow-500 rounded-lg flex items-center justify-center text-white font-bold">
                                        {item.symbol.chars().take(2).collect::<String>()}
                                    </div>
                                    <div>
                                        <div class="font-bold text-white text-lg">{item.symbol}</div>
                                        <div class="text-gray-400 text-sm">Cryptocurrency</div>
                                    </div>
                                </div>
                                <div class="text-right">
                                    <div class="font-bold text-white text-xl">${item.price}</div>
                                    <div class={format!("flex items-center space-x-1 {}", change_class)}>
                                        <span class="text-sm">{icon}</span>
                                        <span class="font-medium">
                                            {if item.change >= 0.0 { "+" } else { "" }}{item.change}%
                                        </span>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                }
            />
        </div>
    }
}
