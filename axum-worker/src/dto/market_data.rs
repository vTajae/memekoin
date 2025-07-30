use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};
use crate::entity::market_data::{MarketDataEvent, Trade, Instrument, TradeSide, InstrumentKind};

/// Request to subscribe to market data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketDataSubscriptionRequest {
    pub exchange: String,
    pub base: String,
    pub quote: String,
    pub data_types: Vec<MarketDataType>,
}

/// Types of market data that can be subscribed to
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarketDataType {
    Trades,
    OrderBook,
    Candles,
}

/// Response for market data subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketDataSubscriptionResponse {
    pub success: bool,
    pub message: String,
    pub subscription_id: Option<String>,
}

/// Market data stream event for API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketDataEventResponse {
    pub event_type: String,
    pub data: MarketDataEventData,
    pub timestamp: DateTime<Utc>,
}

/// Market data event data variants
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MarketDataEventData {
    Trade {
        instrument: InstrumentDto,
        price: Decimal,
        quantity: Decimal,
        side: String,
        exchange_timestamp: DateTime<Utc>,
    },
    OrderBook {
        instrument: InstrumentDto,
        bids: Vec<OrderBookLevelDto>,
        asks: Vec<OrderBookLevelDto>,
        exchange_timestamp: DateTime<Utc>,
    },
    Candle {
        instrument: InstrumentDto,
        open: Decimal,
        high: Decimal,
        low: Decimal,
        close: Decimal,
        volume: Decimal,
        exchange_timestamp: DateTime<Utc>,
    },
}

/// Instrument DTO for API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstrumentDto {
    pub symbol: String,
    pub base: String,
    pub quote: String,
    pub exchange: String,
    pub kind: String,
}

/// Order book level DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookLevelDto {
    pub price: Decimal,
    pub quantity: Decimal,
}

/// Request to get available instruments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInstrumentsRequest {
    pub exchange: Option<String>,
}

/// Response with available instruments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInstrumentsResponse {
    pub instruments: Vec<InstrumentDto>,
}

/// Request to get recent trades
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTradesRequest {
    pub exchange: String,
    pub base: String,
    pub quote: String,
    pub limit: Option<u32>,
}

/// Response with recent trades
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTradesResponse {
    pub trades: Vec<TradeDto>,
}

/// Trade DTO for API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeDto {
    pub id: String,
    pub instrument: InstrumentDto,
    pub price: Decimal,
    pub quantity: Decimal,
    pub side: String,
    pub timestamp: DateTime<Utc>,
}

// Conversion implementations
impl From<&Instrument> for InstrumentDto {
    fn from(instrument: &Instrument) -> Self {
        Self {
            symbol: instrument.symbol.clone(),
            base: instrument.base.clone(),
            quote: instrument.quote.clone(),
            exchange: instrument.exchange.clone(),
            kind: match instrument.instrument_kind {
                InstrumentKind::Spot => "spot".to_string(),
                InstrumentKind::Perpetual => "perpetual".to_string(),
                InstrumentKind::Future => "future".to_string(),
                InstrumentKind::Option => "option".to_string(),
            },
        }
    }
}

impl From<&Trade> for TradeDto {
    fn from(trade: &Trade) -> Self {
        Self {
            id: trade.id.clone(),
            instrument: InstrumentDto::from(&trade.instrument),
            price: trade.price,
            quantity: trade.quantity,
            side: match trade.side {
                TradeSide::Buy => "buy".to_string(),
                TradeSide::Sell => "sell".to_string(),
            },
            timestamp: trade.timestamp,
        }
    }
}

impl From<MarketDataEvent> for MarketDataEventResponse {
    fn from(event: MarketDataEvent) -> Self {
        let (event_type, data) = match event {
            MarketDataEvent::Trade(trade) => (
                "trade".to_string(),
                MarketDataEventData::Trade {
                    instrument: InstrumentDto::from(&trade.instrument),
                    price: trade.price,
                    quantity: trade.quantity,
                    side: match trade.side {
                        TradeSide::Buy => "buy".to_string(),
                        TradeSide::Sell => "sell".to_string(),
                    },
                    exchange_timestamp: trade.exchange_timestamp,
                },
            ),
            MarketDataEvent::OrderBook(order_book) => (
                "orderbook".to_string(),
                MarketDataEventData::OrderBook {
                    instrument: InstrumentDto::from(&order_book.instrument),
                    bids: order_book.bids.iter().map(|level| OrderBookLevelDto {
                        price: level.price,
                        quantity: level.quantity,
                    }).collect(),
                    asks: order_book.asks.iter().map(|level| OrderBookLevelDto {
                        price: level.price,
                        quantity: level.quantity,
                    }).collect(),
                    exchange_timestamp: order_book.exchange_timestamp,
                },
            ),
            MarketDataEvent::Candle(candle) => (
                "candle".to_string(),
                MarketDataEventData::Candle {
                    instrument: InstrumentDto::from(&candle.instrument),
                    open: candle.open,
                    high: candle.high,
                    low: candle.low,
                    close: candle.close,
                    volume: candle.volume,
                    exchange_timestamp: candle.exchange_timestamp,
                },
            ),
        };

        Self {
            event_type,
            data,
            timestamp: Utc::now(),
        }
    }
}
