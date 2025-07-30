use serde_json::{json, Value};
use worker::console_log;

use rust_decimal::Decimal;
use chrono::{DateTime, Utc};

/// Trading side enumeration
#[derive(Debug, Clone, Copy)]
pub enum Side {
    Buy,
    Sell,
}

/// Simple exchange enumeration
#[derive(Debug, Clone, Copy)]
pub enum Exchange {
    Binance,
    BinanceFuturesUsd,
    Coinbase,
    Kraken,
    Okx,
    Bybit,
}

/// Trading Client for interacting with cryptocurrency exchanges using barter-rs
#[derive(Clone)]
pub struct TradingClient {
    pub exchange: Exchange,
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub sandbox_mode: bool,
    pub base_url: String,
}

/// Simple instrument structure for trading
#[derive(Debug, Clone)]
pub struct SimpleInstrument {
    pub base: String,
    pub quote: String,
}

/// Trading order request structure
#[derive(Debug, Clone)]
pub struct OrderRequest {
    pub instrument: SimpleInstrument,
    pub side: Side,
    pub quantity: Decimal,
    pub price: Option<Decimal>, // None for market orders
    pub order_type: OrderType,
}

/// Order type enumeration
#[derive(Debug, Clone)]
pub enum OrderType {
    Market,
    Limit,
    StopLoss,
    TakeProfit,
}

/// Portfolio balance information
#[derive(Debug, Clone)]
pub struct Balance {
    pub asset: String,
    pub free: Decimal,
    pub locked: Decimal,
    pub total: Decimal,
}

/// Market data quote structure
#[derive(Debug, Clone)]
pub struct Quote {
    pub instrument: SimpleInstrument,
    pub bid: Decimal,
    pub ask: Decimal,
    pub timestamp: DateTime<Utc>,
}

/// Order book level
#[derive(Debug, Clone)]
pub struct OrderBookLevel {
    pub price: Decimal,
    pub quantity: Decimal,
}

/// Order book data
#[derive(Debug, Clone)]
pub struct OrderBook {
    pub instrument: SimpleInstrument,
    pub bids: Vec<OrderBookLevel>,
    pub asks: Vec<OrderBookLevel>,
    pub timestamp: DateTime<Utc>,
}

impl TradingClient {
    /// Create a new trading client instance
    pub fn new(exchange: Exchange, api_key: Option<String>, api_secret: Option<String>) -> Self {
        console_log!("TRADING CLIENT: Initializing trading client for exchange: {:?}", exchange);
        
        let base_url = match exchange {
            Exchange::Binance => "https://api.binance.com".to_string(),
            Exchange::BinanceFuturesUsd => "https://fapi.binance.com".to_string(),
            Exchange::Coinbase => "https://api.exchange.coinbase.com".to_string(),
            Exchange::Kraken => "https://api.kraken.com".to_string(),
            Exchange::Okx => "https://www.okx.com".to_string(),
            Exchange::Bybit => "https://api.bybit.com".to_string(),
        };

        Self {
            exchange,
            api_key,
            api_secret,
            sandbox_mode: true, // Start in sandbox mode for safety
            base_url,
        }
    }

    /// Create trading client from environment variables
    pub fn from_env(exchange: Exchange) -> Result<Self, String> {
        console_log!("TRADING CLIENT: Creating client from environment variables for {:?}", exchange);

        // Get API keys from environment variables based on exchange
        let (api_key, api_secret) = match exchange {
            Exchange::Binance => {
                // In production, these would be read from worker environment
                // For now, we'll use placeholder values that should be set in wrangler.toml
                (
                    Some("BINANCE_API_KEY_FROM_ENV".to_string()),
                    Some("BINANCE_API_SECRET_FROM_ENV".to_string())
                )
            }
            Exchange::Coinbase => {
                (
                    Some("COINBASE_API_KEY_FROM_ENV".to_string()),
                    Some("COINBASE_API_SECRET_FROM_ENV".to_string())
                )
            }
            Exchange::Kraken => {
                (
                    Some("KRAKEN_API_KEY_FROM_ENV".to_string()),
                    Some("KRAKEN_API_SECRET_FROM_ENV".to_string())
                )
            }
            _ => {
                console_log!("TRADING CLIENT: No API keys configured for exchange: {:?}", exchange);
                (None, None)
            }
        };

        console_log!("TRADING CLIENT: API keys configured: {}", api_key.is_some() && api_secret.is_some());
        Ok(Self::new(exchange, api_key, api_secret))
    }

    /// Get current market quote for an instrument
    pub async fn get_quote(&self, instrument: &SimpleInstrument) -> Result<Quote, String> {
        console_log!("TRADING CLIENT: Fetching quote for instrument: {:?}", instrument);
        
        // Build API endpoint for the specific exchange
        let endpoint = self.build_quote_endpoint(instrument)?;
        
        // Make HTTP request to exchange API
        match self.make_request(&endpoint, "GET", None).await {
            Ok(response) => {
                console_log!("TRADING CLIENT: Successfully fetched quote data");
                self.parse_quote_response(instrument, &response)
            }
            Err(e) => {
                console_log!("TRADING CLIENT: Failed to fetch quote: {}", e);
                Err(format!("Failed to fetch quote: {}", e))
            }
        }
    }

    /// Get order book data for an instrument
    pub async fn get_order_book(&self, instrument: &SimpleInstrument, depth: u32) -> Result<OrderBook, String> {
        console_log!("TRADING CLIENT: Fetching order book for instrument: {:?} with depth: {}", instrument, depth);
        
        let endpoint = self.build_order_book_endpoint(instrument, depth)?;
        
        match self.make_request(&endpoint, "GET", None).await {
            Ok(response) => {
                console_log!("TRADING CLIENT: Successfully fetched order book data");
                self.parse_order_book_response(instrument, &response)
            }
            Err(e) => {
                console_log!("TRADING CLIENT: Failed to fetch order book: {}", e);
                Err(format!("Failed to fetch order book: {}", e))
            }
        }
    }

    /// Get account balances
    pub async fn get_balances(&self) -> Result<Vec<Balance>, String> {
        console_log!("TRADING CLIENT: Fetching account balances");
        
        if self.api_key.is_none() || self.api_secret.is_none() {
            return Err("API credentials required for balance queries".to_string());
        }

        let endpoint = self.build_balance_endpoint()?;
        
        match self.make_authenticated_request(&endpoint, "GET", None).await {
            Ok(response) => {
                console_log!("TRADING CLIENT: Successfully fetched balance data");
                self.parse_balance_response(&response)
            }
            Err(e) => {
                console_log!("TRADING CLIENT: Failed to fetch balances: {}", e);
                Err(format!("Failed to fetch balances: {}", e))
            }
        }
    }

    /// Place a trading order
    pub async fn place_order(&self, order: &OrderRequest) -> Result<String, String> {
        console_log!("TRADING CLIENT: Placing order: {:?} {} {:?} at {:?}",
            order.side, order.quantity, order.instrument, order.price);
        
        if self.api_key.is_none() || self.api_secret.is_none() {
            return Err("API credentials required for order placement".to_string());
        }

        let endpoint = self.build_order_endpoint()?;
        let payload = self.build_order_payload(order)?;
        
        match self.make_authenticated_request(&endpoint, "POST", Some(payload)).await {
            Ok(response) => {
                console_log!("TRADING CLIENT: Successfully placed order");
                self.parse_order_response(&response)
            }
            Err(e) => {
                console_log!("TRADING CLIENT: Failed to place order: {}", e);
                Err(format!("Failed to place order: {}", e))
            }
        }
    }

    /// Make HTTP request to exchange API
    async fn make_request(&self, endpoint: &str, method: &str, payload: Option<Value>) -> Result<Value, String> {
        console_log!("TRADING CLIENT: Making {} request to: {}", method, endpoint);
        
        // For now, simulate API responses based on the endpoint
        // In production, this would make actual HTTP requests using gloo-net
        self.simulate_api_response(endpoint, method, payload).await
    }

    /// Make authenticated HTTP request to exchange API
    async fn make_authenticated_request(&self, endpoint: &str, method: &str, payload: Option<Value>) -> Result<Value, String> {
        console_log!("TRADING CLIENT: Making authenticated {} request to: {}", method, endpoint);
        
        // Add authentication headers and signature
        // For now, simulate authenticated responses
        self.simulate_api_response(endpoint, method, payload).await
    }

    /// Simulate API responses for development/testing
    async fn simulate_api_response(&self, endpoint: &str, _method: &str, _payload: Option<Value>) -> Result<Value, String> {
        console_log!("TRADING CLIENT: Simulating API response for endpoint: {}", endpoint);
        
        // Generate realistic mock responses based on endpoint
        if endpoint.contains("ticker") || endpoint.contains("quote") {
            Ok(json!({
                "symbol": "BTCUSDT",
                "bidPrice": "45000.50",
                "askPrice": "45001.25",
                "timestamp": Utc::now().timestamp_millis()
            }))
        } else if endpoint.contains("depth") || endpoint.contains("orderbook") {
            Ok(json!({
                "bids": [
                    ["45000.50", "1.25"],
                    ["45000.00", "2.50"],
                    ["44999.75", "0.75"]
                ],
                "asks": [
                    ["45001.25", "1.10"],
                    ["45001.75", "2.25"],
                    ["45002.00", "0.85"]
                ],
                "timestamp": Utc::now().timestamp_millis()
            }))
        } else if endpoint.contains("account") || endpoint.contains("balance") {
            Ok(json!({
                "balances": [
                    {
                        "asset": "BTC",
                        "free": "0.12345678",
                        "locked": "0.00000000"
                    },
                    {
                        "asset": "USDT",
                        "free": "1000.50",
                        "locked": "250.25"
                    }
                ]
            }))
        } else if endpoint.contains("order") {
            Ok(json!({
                "orderId": format!("order_{}", Utc::now().timestamp()),
                "status": "NEW",
                "symbol": "BTCUSDT",
                "side": "BUY",
                "type": "LIMIT",
                "quantity": "0.001",
                "price": "45000.00"
            }))
        } else {
            Err("Unknown endpoint".to_string())
        }
    }

    /// Build quote endpoint URL for the exchange
    fn build_quote_endpoint(&self, instrument: &SimpleInstrument) -> Result<String, String> {
        let symbol = self.format_symbol(instrument)?;
        
        let endpoint = match self.exchange {
            Exchange::Binance => format!("{}/api/v3/ticker/bookTicker?symbol={}", self.base_url, symbol),
            Exchange::Coinbase => format!("{}/products/{}/ticker", self.base_url, symbol),
            Exchange::Kraken => format!("{}/0/public/Ticker?pair={}", self.base_url, symbol),
            _ => format!("{}/api/v3/ticker/bookTicker?symbol={}", self.base_url, symbol),
        };
        
        Ok(endpoint)
    }

    /// Build order book endpoint URL for the exchange
    fn build_order_book_endpoint(&self, instrument: &SimpleInstrument, depth: u32) -> Result<String, String> {
        let symbol = self.format_symbol(instrument)?;
        
        let endpoint = match self.exchange {
            Exchange::Binance => format!("{}/api/v3/depth?symbol={}&limit={}", self.base_url, symbol, depth),
            Exchange::Coinbase => format!("{}/products/{}/book?level=2", self.base_url, symbol),
            Exchange::Kraken => format!("{}/0/public/Depth?pair={}&count={}", self.base_url, symbol, depth),
            _ => format!("{}/api/v3/depth?symbol={}&limit={}", self.base_url, symbol, depth),
        };
        
        Ok(endpoint)
    }

    /// Build balance endpoint URL for the exchange
    fn build_balance_endpoint(&self) -> Result<String, String> {
        let endpoint = match self.exchange {
            Exchange::Binance => format!("{}/api/v3/account", self.base_url),
            Exchange::Coinbase => format!("{}/accounts", self.base_url),
            Exchange::Kraken => format!("{}/0/private/Balance", self.base_url),
            _ => format!("{}/api/v3/account", self.base_url),
        };
        
        Ok(endpoint)
    }

    /// Build order endpoint URL for the exchange
    fn build_order_endpoint(&self) -> Result<String, String> {
        let endpoint = match self.exchange {
            Exchange::Binance => format!("{}/api/v3/order", self.base_url),
            Exchange::Coinbase => format!("{}/orders", self.base_url),
            Exchange::Kraken => format!("{}/0/private/AddOrder", self.base_url),
            _ => format!("{}/api/v3/order", self.base_url),
        };
        
        Ok(endpoint)
    }

    /// Format instrument symbol for the specific exchange
    fn format_symbol(&self, instrument: &SimpleInstrument) -> Result<String, String> {
        match self.exchange {
            Exchange::Binance => Ok(format!("{}{}", instrument.base, instrument.quote)),
            Exchange::Coinbase => Ok(format!("{}-{}", instrument.base, instrument.quote)),
            Exchange::Kraken => Ok(format!("{}{}", instrument.base, instrument.quote)),
            _ => Ok(format!("{}{}", instrument.base, instrument.quote)),
        }
    }

    /// Parse quote response from exchange API
    fn parse_quote_response(&self, instrument: &SimpleInstrument, response: &Value) -> Result<Quote, String> {
        console_log!("TRADING CLIENT: Parsing quote response");

        let bid_str = response["bidPrice"].as_str()
            .or_else(|| response["bid"].as_str())
            .ok_or("Missing bid price in response")?;

        let ask_str = response["askPrice"].as_str()
            .or_else(|| response["ask"].as_str())
            .ok_or("Missing ask price in response")?;

        let bid = Decimal::from_str_exact(bid_str)
            .map_err(|e| format!("Invalid bid price format: {}", e))?;

        let ask = Decimal::from_str_exact(ask_str)
            .map_err(|e| format!("Invalid ask price format: {}", e))?;

        Ok(Quote {
            instrument: instrument.clone(),
            bid,
            ask,
            timestamp: Utc::now(),
        })
    }

    /// Parse order book response from exchange API
    fn parse_order_book_response(&self, instrument: &SimpleInstrument, response: &Value) -> Result<OrderBook, String> {
        console_log!("TRADING CLIENT: Parsing order book response");

        let bids_array = response["bids"].as_array()
            .ok_or("Missing bids in order book response")?;

        let asks_array = response["asks"].as_array()
            .ok_or("Missing asks in order book response")?;

        let mut bids = Vec::new();
        for bid in bids_array {
            if let Some(bid_array) = bid.as_array() {
                if bid_array.len() >= 2 {
                    let price = Decimal::from_str_exact(bid_array[0].as_str().unwrap_or("0"))
                        .map_err(|e| format!("Invalid bid price: {}", e))?;
                    let quantity = Decimal::from_str_exact(bid_array[1].as_str().unwrap_or("0"))
                        .map_err(|e| format!("Invalid bid quantity: {}", e))?;

                    bids.push(OrderBookLevel { price, quantity });
                }
            }
        }

        let mut asks = Vec::new();
        for ask in asks_array {
            if let Some(ask_array) = ask.as_array() {
                if ask_array.len() >= 2 {
                    let price = Decimal::from_str_exact(ask_array[0].as_str().unwrap_or("0"))
                        .map_err(|e| format!("Invalid ask price: {}", e))?;
                    let quantity = Decimal::from_str_exact(ask_array[1].as_str().unwrap_or("0"))
                        .map_err(|e| format!("Invalid ask quantity: {}", e))?;

                    asks.push(OrderBookLevel { price, quantity });
                }
            }
        }

        Ok(OrderBook {
            instrument: instrument.clone(),
            bids,
            asks,
            timestamp: Utc::now(),
        })
    }

    /// Parse balance response from exchange API
    fn parse_balance_response(&self, response: &Value) -> Result<Vec<Balance>, String> {
        console_log!("TRADING CLIENT: Parsing balance response");

        let balances_array = response["balances"].as_array()
            .ok_or("Missing balances in response")?;

        let mut balances = Vec::new();
        for balance in balances_array {
            let asset = balance["asset"].as_str()
                .ok_or("Missing asset in balance")?;

            let free_str = balance["free"].as_str()
                .ok_or("Missing free balance")?;

            let locked_str = balance["locked"].as_str()
                .ok_or("Missing locked balance")?;

            let free = Decimal::from_str_exact(free_str)
                .map_err(|e| format!("Invalid free balance: {}", e))?;

            let locked = Decimal::from_str_exact(locked_str)
                .map_err(|e| format!("Invalid locked balance: {}", e))?;

            let total = free + locked;

            // Only include balances with non-zero amounts
            if total > Decimal::ZERO {
                balances.push(Balance {
                    asset: asset.to_string(),
                    free,
                    locked,
                    total,
                });
            }
        }

        Ok(balances)
    }

    /// Parse order response from exchange API
    fn parse_order_response(&self, response: &Value) -> Result<String, String> {
        console_log!("TRADING CLIENT: Parsing order response");

        let order_id = response["orderId"].as_str()
            .or_else(|| response["id"].as_str())
            .or_else(|| response["order_id"].as_str())
            .ok_or("Missing order ID in response")?;

        Ok(order_id.to_string())
    }

    /// Build order payload for the exchange API
    fn build_order_payload(&self, order: &OrderRequest) -> Result<Value, String> {
        console_log!("TRADING CLIENT: Building order payload");

        let symbol = self.format_symbol(&order.instrument)?;
        let side_str = match order.side {
            Side::Buy => "BUY",
            Side::Sell => "SELL",
        };

        let order_type_str = match order.order_type {
            OrderType::Market => "MARKET",
            OrderType::Limit => "LIMIT",
            OrderType::StopLoss => "STOP_LOSS",
            OrderType::TakeProfit => "TAKE_PROFIT",
        };

        let mut payload = json!({
            "symbol": symbol,
            "side": side_str,
            "type": order_type_str,
            "quantity": order.quantity.to_string(),
            "timestamp": Utc::now().timestamp_millis()
        });

        // Add price for limit orders
        if let Some(price) = order.price {
            payload["price"] = json!(price.to_string());
            payload["timeInForce"] = json!("GTC"); // Good Till Cancelled
        }

        Ok(payload)
    }
}
