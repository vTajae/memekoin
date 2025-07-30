use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};

/// Trading instrument entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingInstrument {
    pub id: String,
    pub symbol: String,
    pub base_asset: String,
    pub quote_asset: String,
    pub exchange: String,
    pub instrument_type: InstrumentType,
    pub status: InstrumentStatus,
    pub min_quantity: Decimal,
    pub max_quantity: Decimal,
    pub quantity_precision: u32,
    pub price_precision: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Type of trading instrument
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstrumentType {
    Spot,
    Futures,
    Perpetual,
    Option,
}

/// Status of trading instrument
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstrumentStatus {
    Active,
    Inactive,
    Delisted,
    PreTrading,
    PostTrading,
}

/// Trading order entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingOrder {
    pub id: String,
    pub user_id: String,
    pub exchange_order_id: Option<String>,
    pub instrument: TradingInstrument,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub status: OrderStatus,
    pub quantity: Decimal,
    pub price: Option<Decimal>,
    pub filled_quantity: Decimal,
    pub average_price: Option<Decimal>,
    pub commission: Decimal,
    pub commission_asset: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub executed_at: Option<DateTime<Utc>>,
}

/// Order side enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderSide {
    Buy,
    Sell,
}

/// Order type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderType {
    Market,
    Limit,
    StopLoss,
    StopLossLimit,
    TakeProfit,
    TakeProfitLimit,
}

/// Order status enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    Cancelled,
    Rejected,
    Expired,
}

/// Portfolio balance entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    pub id: String,
    pub user_id: String,
    pub exchange: String,
    pub balances: Vec<AssetBalance>,
    pub total_value_usd: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Asset balance within a portfolio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetBalance {
    pub asset: String,
    pub free: Decimal,
    pub locked: Decimal,
    pub total: Decimal,
    pub usd_value: Decimal,
}

/// Market quote entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketQuote {
    pub id: String,
    pub instrument: TradingInstrument,
    pub bid_price: Decimal,
    pub ask_price: Decimal,
    pub bid_quantity: Decimal,
    pub ask_quantity: Decimal,
    pub spread: Decimal,
    pub timestamp: DateTime<Utc>,
}

/// Order book entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingOrderBook {
    pub id: String,
    pub instrument: TradingInstrument,
    pub bids: Vec<OrderBookLevel>,
    pub asks: Vec<OrderBookLevel>,
    pub timestamp: DateTime<Utc>,
}

/// Order book level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookLevel {
    pub price: Decimal,
    pub quantity: Decimal,
}

/// Trade execution entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeExecution {
    pub id: String,
    pub order_id: String,
    pub user_id: String,
    pub exchange_trade_id: String,
    pub instrument: TradingInstrument,
    pub side: OrderSide,
    pub quantity: Decimal,
    pub price: Decimal,
    pub commission: Decimal,
    pub commission_asset: String,
    pub is_maker: bool,
    pub executed_at: DateTime<Utc>,
}

/// Trading session entity for tracking user trading activity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingSession {
    pub id: String,
    pub user_id: String,
    pub exchange: String,
    pub status: SessionStatus,
    pub total_trades: u32,
    pub total_volume: Decimal,
    pub realized_pnl: Decimal,
    pub unrealized_pnl: Decimal,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
}

/// Trading session status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionStatus {
    Active,
    Paused,
    Ended,
}

impl TradingInstrument {
    pub fn new(
        symbol: String,
        base_asset: String,
        quote_asset: String,
        exchange: String,
        instrument_type: InstrumentType,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            symbol,
            base_asset,
            quote_asset,
            exchange,
            instrument_type,
            status: InstrumentStatus::Active,
            min_quantity: Decimal::new(1, 8), // 0.00000001
            max_quantity: Decimal::new(1000000, 0), // 1,000,000
            quantity_precision: 8,
            price_precision: 8,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn get_symbol(&self) -> &str {
        &self.symbol
    }

    pub fn is_active(&self) -> bool {
        matches!(self.status, InstrumentStatus::Active)
    }
}

impl TradingOrder {
    pub fn new(
        user_id: String,
        instrument: TradingInstrument,
        side: OrderSide,
        order_type: OrderType,
        quantity: Decimal,
        price: Option<Decimal>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            user_id,
            exchange_order_id: None,
            instrument,
            side,
            order_type,
            status: OrderStatus::New,
            quantity,
            price,
            filled_quantity: Decimal::ZERO,
            average_price: None,
            commission: Decimal::ZERO,
            commission_asset: "USDT".to_string(),
            created_at: now,
            updated_at: now,
            executed_at: None,
        }
    }

    pub fn is_filled(&self) -> bool {
        matches!(self.status, OrderStatus::Filled)
    }

    pub fn is_active(&self) -> bool {
        matches!(self.status, OrderStatus::New | OrderStatus::PartiallyFilled)
    }

    pub fn remaining_quantity(&self) -> Decimal {
        self.quantity - self.filled_quantity
    }
}

impl Portfolio {
    pub fn new(user_id: String, exchange: String) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            user_id,
            exchange,
            balances: Vec::new(),
            total_value_usd: Decimal::ZERO,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn get_balance(&self, asset: &str) -> Option<&AssetBalance> {
        self.balances.iter().find(|b| b.asset == asset)
    }

    pub fn update_balance(&mut self, asset: String, free: Decimal, locked: Decimal, usd_value: Decimal) {
        let total = free + locked;
        
        if let Some(balance) = self.balances.iter_mut().find(|b| b.asset == asset) {
            balance.free = free;
            balance.locked = locked;
            balance.total = total;
            balance.usd_value = usd_value;
        } else {
            self.balances.push(AssetBalance {
                asset,
                free,
                locked,
                total,
                usd_value,
            });
        }
        
        self.total_value_usd = self.balances.iter().map(|b| b.usd_value).sum();
        self.updated_at = Utc::now();
    }
}

impl MarketQuote {
    pub fn new(instrument: TradingInstrument, bid_price: Decimal, ask_price: Decimal) -> Self {
        let spread = ask_price - bid_price;
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            instrument,
            bid_price,
            ask_price,
            bid_quantity: Decimal::ZERO,
            ask_quantity: Decimal::ZERO,
            spread,
            timestamp: Utc::now(),
        }
    }

    pub fn get_mid_price(&self) -> Decimal {
        (self.bid_price + self.ask_price) / Decimal::new(2, 0)
    }

    pub fn get_spread_percentage(&self) -> Decimal {
        if self.bid_price > Decimal::ZERO {
            (self.spread / self.bid_price) * Decimal::new(100, 0)
        } else {
            Decimal::ZERO
        }
    }
}
