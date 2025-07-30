use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};

/// Request to get market quote for an instrument
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetQuoteRequest {
    pub exchange: String,
    pub symbol: String,
}

/// Response containing market quote data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetQuoteResponse {
    pub symbol: String,
    pub exchange: String,
    pub bid_price: Decimal,
    pub ask_price: Decimal,
    pub bid_quantity: Decimal,
    pub ask_quantity: Decimal,
    pub spread: Decimal,
    pub spread_percentage: Decimal,
    pub timestamp: DateTime<Utc>,
}

/// Request to get order book data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOrderBookRequest {
    pub exchange: String,
    pub symbol: String,
    pub depth: Option<u32>,
}

/// Response containing order book data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOrderBookResponse {
    pub symbol: String,
    pub exchange: String,
    pub bids: Vec<OrderBookLevelDto>,
    pub asks: Vec<OrderBookLevelDto>,
    pub timestamp: DateTime<Utc>,
}

/// Order book level DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookLevelDto {
    pub price: Decimal,
    pub quantity: Decimal,
}

/// Request to place a trading order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaceOrderRequest {
    pub exchange: String,
    pub symbol: String,
    pub side: String, // "BUY" or "SELL"
    pub order_type: String, // "MARKET", "LIMIT", etc.
    pub quantity: Decimal,
    pub price: Option<Decimal>,
    pub time_in_force: Option<String>, // "GTC", "IOC", "FOK"
}

/// Response after placing an order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaceOrderResponse {
    pub order_id: String,
    pub exchange_order_id: String,
    pub symbol: String,
    pub side: String,
    pub order_type: String,
    pub status: String,
    pub quantity: Decimal,
    pub price: Option<Decimal>,
    pub filled_quantity: Decimal,
    pub created_at: DateTime<Utc>,
}

/// Request to get account balances
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBalancesRequest {
    pub exchange: String,
}

/// Response containing account balances
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBalancesResponse {
    pub exchange: String,
    pub balances: Vec<BalanceDto>,
    pub total_value_usd: Decimal,
    pub timestamp: DateTime<Utc>,
}

/// Balance DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceDto {
    pub asset: String,
    pub free: Decimal,
    pub locked: Decimal,
    pub total: Decimal,
    pub usd_value: Decimal,
}

/// Request to get trading instruments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInstrumentsRequest {
    pub exchange: String,
    pub instrument_type: Option<String>,
    pub status: Option<String>,
}

/// Response containing trading instruments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInstrumentsResponse {
    pub exchange: String,
    pub instruments: Vec<InstrumentDto>,
}

/// Instrument DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstrumentDto {
    pub symbol: String,
    pub base_asset: String,
    pub quote_asset: String,
    pub exchange: String,
    pub instrument_type: String,
    pub status: String,
    pub min_quantity: Decimal,
    pub max_quantity: Decimal,
    pub quantity_precision: u32,
    pub price_precision: u32,
}

/// Request to get order history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOrderHistoryRequest {
    pub exchange: String,
    pub symbol: Option<String>,
    pub status: Option<String>,
    pub limit: Option<u32>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}

/// Response containing order history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOrderHistoryResponse {
    pub orders: Vec<OrderDto>,
    pub total_count: u32,
}

/// Order DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderDto {
    pub order_id: String,
    pub exchange_order_id: Option<String>,
    pub symbol: String,
    pub side: String,
    pub order_type: String,
    pub status: String,
    pub quantity: Decimal,
    pub price: Option<Decimal>,
    pub filled_quantity: Decimal,
    pub average_price: Option<Decimal>,
    pub commission: Decimal,
    pub commission_asset: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request to cancel an order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelOrderRequest {
    pub exchange: String,
    pub order_id: String,
    pub symbol: String,
}

/// Response after cancelling an order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelOrderResponse {
    pub order_id: String,
    pub status: String,
    pub cancelled_at: DateTime<Utc>,
}

/// Request to get trading status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTradingStatusRequest {
    pub exchange: String,
}

/// Response containing trading status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTradingStatusResponse {
    pub exchange: String,
    pub status: String,
    pub connected: bool,
    pub api_key_valid: bool,
    pub permissions: Vec<String>,
    pub rate_limits: Vec<RateLimitDto>,
    pub server_time: DateTime<Utc>,
}

/// Rate limit DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitDto {
    pub rate_limit_type: String,
    pub interval: String,
    pub interval_num: u32,
    pub limit: u32,
    pub count: u32,
}

/// Request to get portfolio summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPortfolioRequest {
    pub exchange: String,
}

/// Response containing portfolio summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPortfolioResponse {
    pub exchange: String,
    pub total_value_usd: Decimal,
    pub total_pnl_usd: Decimal,
    pub total_pnl_percentage: Decimal,
    pub balances: Vec<BalanceDto>,
    pub top_holdings: Vec<HoldingDto>,
    pub updated_at: DateTime<Utc>,
}

/// Holding DTO for portfolio summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldingDto {
    pub asset: String,
    pub quantity: Decimal,
    pub usd_value: Decimal,
    pub percentage: Decimal,
    pub pnl_usd: Decimal,
    pub pnl_percentage: Decimal,
}

/// Request to get trade history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTradeHistoryRequest {
    pub exchange: String,
    pub symbol: Option<String>,
    pub limit: Option<u32>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}

/// Response containing trade history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTradeHistoryResponse {
    pub trades: Vec<TradeDto>,
    pub total_count: u32,
}

/// Trade DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeDto {
    pub trade_id: String,
    pub order_id: String,
    pub symbol: String,
    pub side: String,
    pub quantity: Decimal,
    pub price: Decimal,
    pub commission: Decimal,
    pub commission_asset: String,
    pub is_maker: bool,
    pub executed_at: DateTime<Utc>,
}

/// Generic error response for trading operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingErrorResponse {
    pub error: String,
    pub error_code: Option<String>,
    pub details: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl TradingErrorResponse {
    pub fn new(error: String) -> Self {
        Self {
            error,
            error_code: None,
            details: None,
            timestamp: Utc::now(),
        }
    }

    pub fn with_code(error: String, error_code: String) -> Self {
        Self {
            error,
            error_code: Some(error_code),
            details: None,
            timestamp: Utc::now(),
        }
    }

    pub fn with_details(error: String, details: String) -> Self {
        Self {
            error,
            error_code: None,
            details: Some(details),
            timestamp: Utc::now(),
        }
    }
}
