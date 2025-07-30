use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};

/// Represents a trading instrument (e.g., BTC/USDT)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instrument {
    pub id: String,
    pub base: String,
    pub quote: String,
    pub exchange: String,
    pub symbol: String,
    pub instrument_kind: InstrumentKind,
}

/// Type of trading instrument
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstrumentKind {
    Spot,
    Perpetual,
    Future,
    Option,
}

/// Market trade data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub id: String,
    pub instrument: Instrument,
    pub price: Decimal,
    pub quantity: Decimal,
    pub side: TradeSide,
    pub timestamp: DateTime<Utc>,
    pub exchange_timestamp: DateTime<Utc>,
}

/// Side of a trade
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeSide {
    Buy,
    Sell,
}

/// Order book level (bid or ask)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookLevel {
    pub price: Decimal,
    pub quantity: Decimal,
}

/// Order book snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    pub instrument: Instrument,
    pub bids: Vec<OrderBookLevel>,
    pub asks: Vec<OrderBookLevel>,
    pub timestamp: DateTime<Utc>,
    pub exchange_timestamp: DateTime<Utc>,
}

/// Candlestick/OHLCV data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candle {
    pub instrument: Instrument,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: Decimal,
    pub timestamp: DateTime<Utc>,
    pub exchange_timestamp: DateTime<Utc>,
}

/// Market data event wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarketDataEvent {
    Trade(Trade),
    OrderBook(OrderBook),
    Candle(Candle),
}

impl Instrument {
    pub fn new(
        base: String,
        quote: String,
        exchange: String,
        instrument_kind: InstrumentKind,
    ) -> Self {
        let symbol = format!("{}/{}", base.to_uppercase(), quote.to_uppercase());
        let id = format!("{}:{}:{}", exchange.to_lowercase(), base.to_lowercase(), quote.to_lowercase());
        
        Self {
            id,
            base: base.to_uppercase(),
            quote: quote.to_uppercase(),
            exchange: exchange.to_lowercase(),
            symbol,
            instrument_kind,
        }
    }
}

impl Trade {
    pub fn new(
        instrument: Instrument,
        price: Decimal,
        quantity: Decimal,
        side: TradeSide,
        exchange_timestamp: DateTime<Utc>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            instrument,
            price,
            quantity,
            side,
            timestamp: Utc::now(),
            exchange_timestamp,
        }
    }
}

impl OrderBook {
    pub fn new(
        instrument: Instrument,
        bids: Vec<OrderBookLevel>,
        asks: Vec<OrderBookLevel>,
        exchange_timestamp: DateTime<Utc>,
    ) -> Self {
        Self {
            instrument,
            bids,
            asks,
            timestamp: Utc::now(),
            exchange_timestamp,
        }
    }
}

impl Candle {
    pub fn new(
        instrument: Instrument,
        open: Decimal,
        high: Decimal,
        low: Decimal,
        close: Decimal,
        volume: Decimal,
        exchange_timestamp: DateTime<Utc>,
    ) -> Self {
        Self {
            instrument,
            open,
            high,
            low,
            close,
            volume,
            timestamp: Utc::now(),
            exchange_timestamp,
        }
    }
}
