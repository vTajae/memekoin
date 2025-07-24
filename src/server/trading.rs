use serde::{Deserialize, Serialize};
use crate::components::ui::trading::*;

// Server function response types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceHistory {
    pub prices: Vec<f64>,
    pub timestamps: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    pub success: bool,
    pub order_id: String,
    pub message: String,
}

// Mock trading server functions for development
// These will be replaced with real implementations using barter-rs

pub async fn get_market_data(symbol: String) -> Result<MarketData, String> {
    // Simulate API delay
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen_futures::JsFuture;
        use web_sys::js_sys;
        let promise = js_sys::Promise::resolve(&js_sys::Number::from(100));
        let _ = JsFuture::from(promise).await;
    }

    // Mock market data
    let base_price = match symbol.as_str() {
        "BTCUSDT" => 45000.0,
        "ETHUSDT" => 3000.0,
        "ADAUSDT" => 0.5,
        "SOLUSDT" => 100.0,
        "DOTUSDT" => 8.0,
        _ => 1.0,
    };

    // Add some randomness
    let random_factor = (js_sys::Math::random() - 0.5) * 0.1; // ±5%
    let current_price = base_price * (1.0 + random_factor);
    let change_24h = base_price * (js_sys::Math::random() - 0.5) * 0.2; // ±10%
    let change_percent_24h = (change_24h / base_price) * 100.0;

    Ok(MarketData {
        symbol,
        price: current_price,
        change_24h,
        change_percent_24h,
        volume_24h: 1000000.0 + js_sys::Math::random() * 5000000.0,
        high_24h: current_price * 1.05,
        low_24h: current_price * 0.95,
        timestamp: chrono::Utc::now().timestamp_millis(),
    })
}

pub async fn get_order_book(symbol: String) -> Result<OrderBook, String> {
    // Mock order book data
    let base_price = match symbol.as_str() {
        "BTCUSDT" => 45000.0,
        "ETHUSDT" => 3000.0,
        "ADAUSDT" => 0.5,
        "SOLUSDT" => 100.0,
        "DOTUSDT" => 8.0,
        _ => 1.0,
    };

    let mut bids = Vec::new();
    let mut asks = Vec::new();

    // Generate mock bids (buy orders)
    for i in 0..10 {
        let price = base_price * (1.0 - (i as f64 * 0.001));
        let quantity = 1.0 + js_sys::Math::random() * 10.0;
        bids.push(OrderBookEntry {
            price,
            quantity,
            total: price * quantity,
        });
    }

    // Generate mock asks (sell orders)
    for i in 0..10 {
        let price = base_price * (1.0 + (i as f64 * 0.001));
        let quantity = 1.0 + js_sys::Math::random() * 10.0;
        asks.push(OrderBookEntry {
            price,
            quantity,
            total: price * quantity,
        });
    }

    Ok(OrderBook {
        symbol,
        bids,
        asks,
        timestamp: chrono::Utc::now().timestamp_millis(),
    })
}

pub async fn get_portfolio() -> Result<Portfolio, String> {
    // Mock portfolio data
    let positions = vec![
        Position {
            symbol: "BTCUSDT".to_string(),
            quantity: 0.5,
            average_price: 44000.0,
            current_price: 45000.0,
            pnl: 500.0,
            pnl_percent: 2.27,
            side: PositionSide::Long,
        },
        Position {
            symbol: "ETHUSDT".to_string(),
            quantity: 2.0,
            average_price: 2950.0,
            current_price: 3000.0,
            pnl: 100.0,
            pnl_percent: 1.69,
            side: PositionSide::Long,
        },
    ];

    let total_value = 50000.0;
    let available_balance = 10000.0;
    let pnl_24h = 600.0;
    let pnl_percent_24h = 1.2;

    Ok(Portfolio {
        total_value,
        available_balance,
        positions,
        pnl_24h,
        pnl_percent_24h,
    })
}

pub async fn get_price_history(symbol: String, timeframe: String) -> Result<PriceHistory, String> {
    let base_price = match symbol.as_str() {
        "BTCUSDT" => 45000.0,
        "ETHUSDT" => 3000.0,
        "ADAUSDT" => 0.5,
        "SOLUSDT" => 100.0,
        "DOTUSDT" => 8.0,
        _ => 1.0,
    };

    let points = match timeframe.as_str() {
        "1h" => 60,
        "4h" => 96,
        "1d" => 24,
        "1w" => 168,
        _ => 60,
    };

    let mut prices = Vec::new();
    let mut timestamps = Vec::new();
    let now = chrono::Utc::now().timestamp_millis();

    for i in 0..points {
        let time_offset = (points - i) as i64 * 60000; // 1 minute intervals
        let timestamp = now - time_offset;
        
        // Generate realistic price movement
        let random_change = (js_sys::Math::random() - 0.5) * 0.02; // ±1%
        let price = base_price * (1.0 + random_change);
        
        prices.push(price);
        timestamps.push(timestamp);
    }

    Ok(PriceHistory { prices, timestamps })
}

pub async fn submit_order(order: Order) -> Result<OrderResponse, String> {
    // Simulate order processing delay
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen_futures::JsFuture;
        use web_sys::js_sys;
        let promise = js_sys::Promise::resolve(&js_sys::Number::from(500));
        let _ = JsFuture::from(promise).await;
    }

    // Mock order validation
    if order.quantity <= 0.0 {
        return Err("Invalid quantity".to_string());
    }

    if matches!(order.order_type, OrderType::Limit) && order.price.is_none() {
        return Err("Price required for limit orders".to_string());
    }

    // Simulate random order success/failure
    if js_sys::Math::random() > 0.1 { // 90% success rate
        Ok(OrderResponse {
            success: true,
            order_id: order.id,
            message: "Order submitted successfully".to_string(),
        })
    } else {
        Err("Order rejected: Insufficient balance".to_string())
    }
}

pub async fn get_market_overview() -> Result<Vec<MarketData>, String> {
    let symbols = vec!["BTCUSDT", "ETHUSDT", "ADAUSDT", "SOLUSDT", "DOTUSDT", "LINKUSDT", "AVAXUSDT", "MATICUSDT"];
    
    let mut markets = Vec::new();
    for symbol in symbols {
        if let Ok(market_data) = get_market_data(symbol.to_string()).await {
            markets.push(market_data);
        }
    }
    
    Ok(markets)
}
