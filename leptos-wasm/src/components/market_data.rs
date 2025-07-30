use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instrument {
    pub symbol: String,
    pub base: String,
    pub quote: String,
    pub exchange: String,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketDataStatus {
    pub status: String,
    pub active_subscriptions: u32,
    pub service: String,
    pub barter_integration: String,
}

#[component]
pub fn MarketDataDashboard() -> impl IntoView {
    let (instruments, set_instruments) = signal(Vec::<Instrument>::new());
    let (status, set_status) = signal(None::<MarketDataStatus>);
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal(None::<String>);

    // Load market data status on component mount
    Effect::new(move |_| {
        spawn_local(async move {
            set_loading.set(true);
            
            // Fetch market data status
            match fetch_market_data_status().await {
                Ok(status_data) => {
                    set_status.set(Some(status_data));
                    set_error.set(None);
                }
                Err(e) => {
                    set_error.set(Some(format!("Failed to load status: {}", e)));
                }
            }
            
            // Fetch available instruments
            match fetch_instruments().await {
                Ok(instruments_data) => {
                    set_instruments.set(instruments_data);
                }
                Err(e) => {
                    set_error.set(Some(format!("Failed to load instruments: {}", e)));
                }
            }
            
            set_loading.set(false);
        });
    });

    let subscribe_to_market_data = move |exchange: &str, base: &str, quote: &str| {
        let exchange = exchange.to_string();
        let base = base.to_string();
        let quote = quote.to_string();
        
        spawn_local(async move {
            match subscribe_to_trades(&exchange, &base, &quote).await {
                Ok(_) => {
                    web_sys::console::log_1(&format!("Subscribed to {}/{} on {}", base, quote, exchange).into());
                }
                Err(e) => {
                    web_sys::console::log_1(&format!("Subscription failed: {}", e).into());
                }
            }
        });
    };

    view! {
        <div class="bg-white shadow rounded-lg p-6">
            <h2 class="text-2xl font-bold text-gray-900 mb-6">
                "Market Data Dashboard"
                <span class="text-sm font-normal text-gray-500 ml-2">
                    "Powered by Barter-rs"
                </span>
            </h2>

            // Status Section
            <div class="mb-6">
                <h3 class="text-lg font-semibold text-gray-800 mb-3">"Service Status"</h3>
                {move || {
                    if loading.get() {
                        view! {
                            <div class="flex items-center">
                                <div class="animate-spin rounded-full h-4 w-4 border-b-2 border-blue-600 mr-2"></div>
                                <span class="text-gray-600">"Loading..."</span>
                            </div>
                        }.into_any()
                    } else if let Some(error_msg) = error.get() {
                        view! {
                            <div class="bg-red-50 border border-red-200 rounded-md p-3">
                                <p class="text-red-800 text-sm">{error_msg}</p>
                            </div>
                        }.into_any()
                    } else if let Some(status_data) = status.get() {
                        view! {
                            <div class="bg-green-50 border border-green-200 rounded-md p-3">
                                <div class="flex items-center justify-between">
                                    <div>
                                        <p class="text-green-800 font-medium">
                                            "Status: " {status_data.status}
                                        </p>
                                        <p class="text-green-700 text-sm">
                                            "Active Subscriptions: " {status_data.active_subscriptions}
                                        </p>
                                    </div>
                                    <div class="text-right">
                                        <p class="text-green-700 text-sm">
                                            "Integration: " {status_data.barter_integration}
                                        </p>
                                    </div>
                                </div>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="text-gray-500">"No status available"</div>
                        }.into_any()
                    }
                }}
            </div>

            // Instruments Section
            <div class="mb-6">
                <h3 class="text-lg font-semibold text-gray-800 mb-3">"Available Instruments"</h3>
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                    <For
                        each=move || instruments.get()
                        key=|instrument| format!("{}:{}", instrument.exchange, instrument.symbol)
                        children=move |instrument| {
                            let exchange = instrument.exchange.clone();
                            let base = instrument.base.clone();
                            let quote = instrument.quote.clone();
                            
                            view! {
                                <div class="border border-gray-200 rounded-lg p-4 hover:shadow-md transition-shadow">
                                    <div class="flex justify-between items-start mb-2">
                                        <h4 class="font-semibold text-gray-900">{instrument.symbol.clone()}</h4>
                                        <span class="text-xs bg-blue-100 text-blue-800 px-2 py-1 rounded">
                                            {instrument.kind.clone()}
                                        </span>
                                    </div>
                                    <p class="text-sm text-gray-600 mb-3">
                                        "Exchange: " {instrument.exchange.clone()}
                                    </p>
                                    <button
                                        class="w-full bg-blue-600 text-white text-sm py-2 px-3 rounded hover:bg-blue-700 transition-colors"
                                        on:click=move |_| {
                                            subscribe_to_market_data(&exchange, &base, &quote);
                                        }
                                    >
                                        "Subscribe to Trades"
                                    </button>
                                </div>
                            }
                        }
                    />
                </div>
                {move || {
                    if instruments.get().is_empty() && !loading.get() {
                        view! {
                            <div class="text-center py-8 text-gray-500">
                                "No instruments available"
                            </div>
                        }.into_any()
                    } else {
                        view! { <div></div> }.into_any()
                    }
                }}
            </div>

            // Info Section
            <div class="bg-blue-50 border border-blue-200 rounded-md p-4">
                <h4 class="font-semibold text-blue-900 mb-2">"About Barter-rs Integration"</h4>
                <p class="text-blue-800 text-sm">
                    "This is a basic integration with the Barter-rs trading framework. "
                    "It demonstrates market data streaming capabilities and instrument management. "
                    "Click 'Subscribe to Trades' to start receiving live market data for an instrument."
                </p>
            </div>
        </div>
    }
}

// API functions
async fn fetch_market_data_status() -> Result<MarketDataStatus, String> {
    let response = gloo_net::http::Request::get("/api/market-data/status")
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if response.ok() {
        response
            .json::<MarketDataStatus>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    } else {
        Err(format!("HTTP error: {}", response.status()))
    }
}

async fn fetch_instruments() -> Result<Vec<Instrument>, String> {
    let response = gloo_net::http::Request::post("/api/market-data/instruments")
        .json(&serde_json::json!({}))
        .map_err(|e| format!("Failed to serialize request: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if response.ok() {
        #[derive(Deserialize)]
        struct InstrumentsResponse {
            instruments: Vec<Instrument>,
        }
        
        let instruments_response: InstrumentsResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
            
        Ok(instruments_response.instruments)
    } else {
        Err(format!("HTTP error: {}", response.status()))
    }
}

async fn subscribe_to_trades(exchange: &str, base: &str, quote: &str) -> Result<(), String> {
    let request = serde_json::json!({
        "exchange": exchange,
        "base": base,
        "quote": quote,
        "data_types": ["Trades"]
    });

    let response = gloo_net::http::Request::post("/api/market-data/subscribe")
        .json(&request)
        .map_err(|e| format!("Failed to serialize request: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if response.ok() {
        Ok(())
    } else {
        Err(format!("HTTP error: {}", response.status()))
    }
}
