use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use worker::console_log;
use rust_decimal::Decimal;
use chrono::Utc;

use crate::entity::market_data::{
    Instrument, InstrumentKind, Trade, TradeSide
};
use crate::dto::market_data::{
    MarketDataSubscriptionRequest, MarketDataSubscriptionResponse,
    GetInstrumentsResponse, InstrumentDto
};

/// Market data service inspired by barter-rs architecture but WASM-compatible
#[derive(Clone)]
pub struct MarketDataService {
    // Store active subscriptions (with interior mutability for WASM)
    subscriptions: Rc<RefCell<HashMap<String, MarketDataSubscription>>>,
    // Available instruments
    instruments: Vec<Instrument>,
    // Service status
    is_ready: bool,
}

/// Represents an active market data subscription
#[derive(Debug, Clone)]
pub struct MarketDataSubscription {
    pub id: String,
    pub exchange: String,
    pub base: String,
    pub quote: String,
    pub data_types: Vec<String>,
    pub active: bool,
}

impl MarketDataService {
    pub fn new() -> Self {
        let mut instruments = Vec::new();

        // Add some demo instruments for testing (inspired by barter-rs patterns)
        instruments.push(Instrument::new(
            "BTC".to_string(),
            "USDT".to_string(),
            "binance".to_string(),
            InstrumentKind::Spot,
        ));

        instruments.push(Instrument::new(
            "ETH".to_string(),
            "USDT".to_string(),
            "binance".to_string(),
            InstrumentKind::Spot,
        ));

        instruments.push(Instrument::new(
            "BTC".to_string(),
            "USD".to_string(),
            "coinbase".to_string(),
            InstrumentKind::Spot,
        ));

        console_log!("MARKET DATA: Initialized service with {} instruments (barter-rs inspired)", instruments.len());

        Self {
            subscriptions: Rc::new(RefCell::new(HashMap::new())),
            instruments,
            is_ready: true,
        }
    }

    /// Subscribe to market data for a specific instrument (barter-rs inspired)
    pub async fn subscribe(&self, request: MarketDataSubscriptionRequest) -> Result<MarketDataSubscriptionResponse, String> {
        console_log!("MARKET DATA: Subscribing to {} {}/{} on {} (barter-rs style)",
            request.data_types.len(), request.base, request.quote, request.exchange);

        let subscription_id = uuid::Uuid::new_v4().to_string();

        let subscription = MarketDataSubscription {
            id: subscription_id.clone(),
            exchange: request.exchange.clone(),
            base: request.base.clone(),
            quote: request.quote.clone(),
            data_types: request.data_types.iter().map(|dt| format!("{:?}", dt)).collect(),
            active: true,
        };

        // Store the subscription (with interior mutability for WASM)
        match self.subscriptions.try_borrow_mut() {
            Ok(mut subscriptions) => {
                subscriptions.insert(subscription_id.clone(), subscription);
            }
            Err(_) => {
                return Err("Failed to acquire mutable borrow on subscriptions".to_string());
            }
        }

        console_log!("MARKET DATA: Subscription {} created successfully (following barter-rs patterns)", subscription_id);

        Ok(MarketDataSubscriptionResponse {
            success: true,
            message: "Subscription created successfully (barter-rs inspired)".to_string(),
            subscription_id: Some(subscription_id),
        })
    }

    /// Get available instruments (barter-rs inspired)
    pub async fn get_instruments(&self, exchange_filter: Option<String>) -> Result<GetInstrumentsResponse, String> {
        console_log!("MARKET DATA: Getting instruments for exchange: {:?} (barter-rs style)", exchange_filter);

        let filtered_instruments: Vec<InstrumentDto> = self.instruments
            .iter()
            .filter(|instrument| {
                exchange_filter.as_ref().map_or(true, |exchange| &instrument.exchange == exchange)
            })
            .map(|instrument| InstrumentDto::from(instrument))
            .collect();

        console_log!("MARKET DATA: Found {} instruments (following barter-rs patterns)", filtered_instruments.len());

        Ok(GetInstrumentsResponse {
            instruments: filtered_instruments,
        })
    }

    /// Generate a sample trade for demo purposes (barter-rs inspired)
    pub fn generate_sample_trade(&self, exchange: &str, base: &str, quote: &str) -> Result<Trade, String> {
        console_log!("MARKET DATA: Generating sample trade for {}/{} on {} (barter-rs style)", base, quote, exchange);

        let instrument = Instrument::new(
            base.to_string(),
            quote.to_string(),
            exchange.to_string(),
            InstrumentKind::Spot,
        );

        // Generate mock trade data (following barter-rs patterns)
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        Utc::now().timestamp_nanos_opt().unwrap_or(0).hash(&mut hasher);
        let random_seed = hasher.finish();

        let price = Decimal::new(50000 + ((random_seed % 1000) as i64), 0);
        let quantity = Decimal::new(100 + ((random_seed % 50) as i64), 3);
        let side = if random_seed % 2 == 0 { TradeSide::Buy } else { TradeSide::Sell };

        let trade = Trade::new(
            instrument,
            price,
            quantity,
            side,
            Utc::now(),
        );

        console_log!("MARKET DATA: Generated sample trade at price {} (barter-rs inspired)", price);
        Ok(trade)
    }

    /// Get active subscriptions count (barter-rs inspired)
    pub async fn get_active_subscriptions_count(&self) -> usize {
        console_log!("MARKET DATA: Getting active subscriptions count (barter-rs style)");
        match self.subscriptions.try_borrow() {
            Ok(subscriptions) => subscriptions.len(),
            Err(_) => 0,
        }
    }

    /// Check if service is ready (barter-rs inspired)
    pub async fn is_ready(&self) -> bool {
        console_log!("MARKET DATA: Checking service readiness (barter-rs style)");
        self.is_ready && !self.instruments.is_empty()
    }
}

impl Default for MarketDataService {
    fn default() -> Self {
        Self::new()
    }
}
