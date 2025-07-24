use leptos::prelude::*;

#[component]
pub fn TradingInterface() -> impl IntoView {
    let (selected_symbol, set_selected_symbol) = signal("BTC".to_string());
    let (amount, set_amount) = signal(0.0);
    let (order_type, set_order_type) = signal("buy".to_string());

    let execute_trade = move |_| {
        // This would be a server function in a real implementation
        leptos::logging::log!("Executing {} order for {} {}", order_type.get(), amount.get(), selected_symbol.get());
    };

    view! {
        <div id="trading-interface">
            <div class="space-y-6">
                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                    <div class="space-y-2">
                        <label class="block text-sm font-medium text-gray-300 mb-2">Trading Pair</label>
                        <div class="relative">
                            <select
                                class="w-full bg-white/10 border border-white/20 rounded-xl px-4 py-3 text-white focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent appearance-none cursor-pointer"
                                on:change=move |ev| {
                                    set_selected_symbol.set(event_target_value(&ev));
                                }
                            >
                                <option value="BTC">"🟠 Bitcoin (BTC)"</option>
                                <option value="ETH">"🔷 Ethereum (ETH)"</option>
                                <option value="DOGE">"🐕 Dogecoin (DOGE)"</option>
                            </select>
                            <div class="absolute inset-y-0 right-0 flex items-center pr-3 pointer-events-none">
                                <svg class="w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
                                </svg>
                            </div>
                        </div>
                    </div>
                    <div class="space-y-2">
                        <label class="block text-sm font-medium text-gray-300 mb-2">Amount</label>
                        <div class="relative">
                            <input
                                type="number"
                                step="0.01"
                                placeholder="0.00"
                                class="w-full bg-white/10 border border-white/20 rounded-xl px-4 py-3 text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                                prop:value=amount
                                on:input=move |ev| {
                                    if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                                        set_amount.set(val);
                                    }
                                }
                            />
                            <div class="absolute inset-y-0 right-0 flex items-center pr-3">
                                <span class="text-gray-400 text-sm">USD</span>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="grid grid-cols-2 gap-4">
                    <button
                        class="group relative bg-gradient-to-r from-green-500 to-emerald-600 text-white font-semibold py-4 px-6 rounded-xl hover:from-green-600 hover:to-emerald-700 transform hover:scale-105 transition-all duration-200 shadow-lg hover:shadow-green-500/25"
                        on:click=move |_| {
                            set_order_type.set("buy".to_string());
                            execute_trade(());
                        }
                    >
                        <div class="flex items-center justify-center space-x-2">
                            <span class="text-lg">"📈"</span>
                            <span>Buy</span>
                        </div>
                        <div class="absolute inset-0 bg-white/20 rounded-xl opacity-0 group-hover:opacity-100 transition-opacity duration-200"></div>
                    </button>
                    <button
                        class="group relative bg-gradient-to-r from-red-500 to-rose-600 text-white font-semibold py-4 px-6 rounded-xl hover:from-red-600 hover:to-rose-700 transform hover:scale-105 transition-all duration-200 shadow-lg hover:shadow-red-500/25"
                        on:click=move |_| {
                            set_order_type.set("sell".to_string());
                            execute_trade(());
                        }
                    >
                        <div class="flex items-center justify-center space-x-2">
                            <span class="text-lg">"📉"</span>
                            <span>Sell</span>
                        </div>
                        <div class="absolute inset-0 bg-white/20 rounded-xl opacity-0 group-hover:opacity-100 transition-opacity duration-200"></div>
                    </button>
                </div>

                <div class="bg-blue-500/10 border border-blue-500/20 rounded-xl p-4">
                    <div class="flex items-center space-x-2 text-blue-400">
                        <span class="text-lg">"💡"</span>
                        <span class="font-medium">Quick Trade</span>
                    </div>
                    <p class="text-gray-300 text-sm mt-2">
                        Execute trades instantly with our advanced algorithm. Market orders are processed in real-time.
                    </p>
                </div>
            </div>
        </div>
    }
}
