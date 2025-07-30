use worker::console_log;
use std::collections::HashMap;
use rust_decimal::Decimal;
use chrono::Utc;



use crate::clients::trading::{TradingClient, OrderRequest, OrderType, Quote, OrderBook, Balance, SimpleInstrument, Exchange, Side};
use crate::dto::trading::{
    GetQuoteRequest, GetQuoteResponse, GetOrderBookRequest, GetOrderBookResponse,
    PlaceOrderRequest, PlaceOrderResponse, GetBalancesRequest, GetBalancesResponse,
    GetInstrumentsRequest, GetInstrumentsResponse, GetTradingStatusRequest, GetTradingStatusResponse,
    OrderBookLevelDto, BalanceDto, InstrumentDto, TradingErrorResponse
};


/// Trading service that orchestrates trading operations using barter-rs
#[derive(Clone)]
pub struct TradingService {
    clients: HashMap<String, TradingClient>,
    supported_exchanges: Vec<Exchange>,
}

impl TradingService {
    /// Create a new trading service instance
    pub fn new() -> Self {
        console_log!("TRADING SERVICE: Initializing trading service with barter-rs integration");
        
        let mut service = Self {
            clients: HashMap::new(),
            supported_exchanges: vec![
                Exchange::Binance,
                Exchange::Coinbase,
                Exchange::Kraken,
                Exchange::Okx,
                Exchange::Bybit,
            ],
        };
        
        // Initialize clients for supported exchanges
        service.initialize_clients();
        
        service
    }

    /// Initialize trading clients for supported exchanges
    fn initialize_clients(&mut self) {
        console_log!("TRADING SERVICE: Initializing trading clients for supported exchanges");
        
        for exchange in &self.supported_exchanges {
            match TradingClient::from_env(*exchange) {
                Ok(client) => {
                    let exchange_name = format!("{:?}", exchange).to_lowercase();
                    self.clients.insert(exchange_name, client);
                    console_log!("TRADING SERVICE: Successfully initialized client for {:?}", exchange);
                }
                Err(e) => {
                    console_log!("TRADING SERVICE: Failed to initialize client for {:?}: {}", exchange, e);
                }
            }
        }
        
        console_log!("TRADING SERVICE: Initialized {} trading clients", self.clients.len());
    }

    /// Get market quote for an instrument
    pub async fn get_quote(&self, request: GetQuoteRequest) -> Result<GetQuoteResponse, TradingErrorResponse> {
        console_log!("TRADING SERVICE: Getting quote for {} on {}", request.symbol, request.exchange);
        
        let client = self.get_client(&request.exchange)?;
        let instrument = self.parse_instrument(&request.symbol, &request.exchange)?;
        
        match client.get_quote(&instrument).await {
            Ok(quote) => {
                console_log!("TRADING SERVICE: Successfully retrieved quote for {}", request.symbol);
                Ok(self.convert_quote_to_response(quote, &request.exchange))
            }
            Err(e) => {
                console_log!("TRADING SERVICE: Failed to get quote: {}", e);
                Err(TradingErrorResponse::new(format!("Failed to get quote: {}", e)))
            }
        }
    }

    /// Get order book data for an instrument
    pub async fn get_order_book(&self, request: GetOrderBookRequest) -> Result<GetOrderBookResponse, TradingErrorResponse> {
        console_log!("TRADING SERVICE: Getting order book for {} on {}", request.symbol, request.exchange);
        
        let client = self.get_client(&request.exchange)?;
        let instrument = self.parse_instrument(&request.symbol, &request.exchange)?;
        let depth = request.depth.unwrap_or(20);
        
        match client.get_order_book(&instrument, depth).await {
            Ok(order_book) => {
                console_log!("TRADING SERVICE: Successfully retrieved order book for {}", request.symbol);
                Ok(self.convert_order_book_to_response(order_book, &request.exchange))
            }
            Err(e) => {
                console_log!("TRADING SERVICE: Failed to get order book: {}", e);
                Err(TradingErrorResponse::new(format!("Failed to get order book: {}", e)))
            }
        }
    }

    /// Place a trading order
    pub async fn place_order(&self, request: PlaceOrderRequest) -> Result<PlaceOrderResponse, TradingErrorResponse> {
        console_log!("TRADING SERVICE: Placing {} order for {} {} on {}", 
            request.side, request.quantity, request.symbol, request.exchange);
        
        let client = self.get_client(&request.exchange)?;
        let order_request = self.convert_place_order_request(request)?;
        
        match client.place_order(&order_request).await {
            Ok(order_id) => {
                console_log!("TRADING SERVICE: Successfully placed order with ID: {}", order_id);
                Ok(PlaceOrderResponse {
                    order_id: uuid::Uuid::new_v4().to_string(),
                    exchange_order_id: order_id,
                    symbol: order_request.instrument.base.clone() + &order_request.instrument.quote,
                    side: format!("{:?}", order_request.side),
                    order_type: format!("{:?}", order_request.order_type),
                    status: "NEW".to_string(),
                    quantity: order_request.quantity,
                    price: order_request.price,
                    filled_quantity: Decimal::ZERO,
                    created_at: Utc::now(),
                })
            }
            Err(e) => {
                console_log!("TRADING SERVICE: Failed to place order: {}", e);
                Err(TradingErrorResponse::new(format!("Failed to place order: {}", e)))
            }
        }
    }

    /// Get account balances
    pub async fn get_balances(&self, request: GetBalancesRequest) -> Result<GetBalancesResponse, TradingErrorResponse> {
        console_log!("TRADING SERVICE: Getting balances for {}", request.exchange);
        
        let client = self.get_client(&request.exchange)?;
        
        match client.get_balances().await {
            Ok(balances) => {
                console_log!("TRADING SERVICE: Successfully retrieved {} balances", balances.len());
                Ok(self.convert_balances_to_response(balances, &request.exchange))
            }
            Err(e) => {
                console_log!("TRADING SERVICE: Failed to get balances: {}", e);
                Err(TradingErrorResponse::new(format!("Failed to get balances: {}", e)))
            }
        }
    }

    /// Get available trading instruments
    pub async fn get_instruments(&self, request: GetInstrumentsRequest) -> Result<GetInstrumentsResponse, TradingErrorResponse> {
        console_log!("TRADING SERVICE: Getting instruments for {}", request.exchange);
        
        // For now, return a predefined list of popular instruments
        // In production, this would fetch from the exchange API
        let instruments = self.get_popular_instruments(&request.exchange);
        
        console_log!("TRADING SERVICE: Returning {} instruments for {}", instruments.len(), request.exchange);
        
        Ok(GetInstrumentsResponse {
            exchange: request.exchange,
            instruments,
        })
    }

    /// Get trading status for an exchange
    pub async fn get_trading_status(&self, request: GetTradingStatusRequest) -> Result<GetTradingStatusResponse, TradingErrorResponse> {
        console_log!("TRADING SERVICE: Getting trading status for {}", request.exchange);
        
        let client = self.get_client(&request.exchange)?;
        
        // Check if client is properly configured
        let api_key_valid = client.api_key.is_some() && client.api_secret.is_some();
        let connected = true; // In production, this would test the connection
        
        console_log!("TRADING SERVICE: Trading status - Connected: {}, API Key Valid: {}", connected, api_key_valid);
        
        Ok(GetTradingStatusResponse {
            exchange: request.exchange,
            status: if connected && api_key_valid { "ACTIVE" } else { "INACTIVE" }.to_string(),
            connected,
            api_key_valid,
            permissions: vec!["SPOT".to_string(), "MARGIN".to_string()],
            rate_limits: vec![], // Would be populated from exchange info
            server_time: Utc::now(),
        })
    }

    /// Get trading client for a specific exchange
    fn get_client(&self, exchange: &str) -> Result<&TradingClient, TradingErrorResponse> {
        self.clients.get(exchange)
            .ok_or_else(|| TradingErrorResponse::new(format!("Unsupported exchange: {}", exchange)))
    }

    /// Parse instrument symbol into SimpleInstrument
    fn parse_instrument(&self, symbol: &str, _exchange: &str) -> Result<SimpleInstrument, TradingErrorResponse> {
        // Parse symbol like "BTCUSDT" into base and quote
        let (base, quote) = self.parse_symbol(symbol)?;

        Ok(SimpleInstrument {
            base,
            quote,
        })
    }

    /// Parse trading symbol into base and quote assets
    fn parse_symbol(&self, symbol: &str) -> Result<(String, String), TradingErrorResponse> {
        // Common quote currencies to try
        let quote_currencies = ["USDT", "USDC", "BTC", "ETH", "BNB", "USD", "EUR"];
        
        for quote in &quote_currencies {
            if symbol.ends_with(quote) {
                let base = symbol.strip_suffix(quote).unwrap();
                if !base.is_empty() {
                    return Ok((base.to_string(), quote.to_string()));
                }
            }
        }
        
        Err(TradingErrorResponse::new(format!("Unable to parse symbol: {}", symbol)))
    }

    /// Convert barter-rs Quote to response DTO
    fn convert_quote_to_response(&self, quote: Quote, exchange: &str) -> GetQuoteResponse {
        let spread = quote.ask - quote.bid;
        let spread_percentage = if quote.bid > Decimal::ZERO {
            (spread / quote.bid) * Decimal::new(100, 0)
        } else {
            Decimal::ZERO
        };
        
        GetQuoteResponse {
            symbol: format!("{}{}", quote.instrument.base, quote.instrument.quote),
            exchange: exchange.to_string(),
            bid_price: quote.bid,
            ask_price: quote.ask,
            bid_quantity: Decimal::ZERO, // Would be populated from actual quote
            ask_quantity: Decimal::ZERO, // Would be populated from actual quote
            spread,
            spread_percentage,
            timestamp: quote.timestamp,
        }
    }

    /// Convert barter-rs OrderBook to response DTO
    fn convert_order_book_to_response(&self, order_book: OrderBook, exchange: &str) -> GetOrderBookResponse {
        let bids: Vec<OrderBookLevelDto> = order_book.bids.into_iter()
            .map(|level| OrderBookLevelDto {
                price: level.price,
                quantity: level.quantity,
            })
            .collect();
        
        let asks: Vec<OrderBookLevelDto> = order_book.asks.into_iter()
            .map(|level| OrderBookLevelDto {
                price: level.price,
                quantity: level.quantity,
            })
            .collect();
        
        GetOrderBookResponse {
            symbol: format!("{}{}", order_book.instrument.base, order_book.instrument.quote),
            exchange: exchange.to_string(),
            bids,
            asks,
            timestamp: order_book.timestamp,
        }
    }

    /// Convert place order request to barter-rs OrderRequest
    fn convert_place_order_request(&self, request: PlaceOrderRequest) -> Result<OrderRequest, TradingErrorResponse> {
        let instrument = self.parse_instrument(&request.symbol, &request.exchange)?;
        
        let side = match request.side.to_uppercase().as_str() {
            "BUY" => Side::Buy,
            "SELL" => Side::Sell,
            _ => return Err(TradingErrorResponse::new(format!("Invalid order side: {}", request.side))),
        };
        
        let order_type = match request.order_type.to_uppercase().as_str() {
            "MARKET" => OrderType::Market,
            "LIMIT" => OrderType::Limit,
            "STOP_LOSS" => OrderType::StopLoss,
            "TAKE_PROFIT" => OrderType::TakeProfit,
            _ => return Err(TradingErrorResponse::new(format!("Invalid order type: {}", request.order_type))),
        };
        
        Ok(OrderRequest {
            instrument,
            side,
            quantity: request.quantity,
            price: request.price,
            order_type,
        })
    }

    /// Convert barter-rs balances to response DTO
    fn convert_balances_to_response(&self, balances: Vec<Balance>, exchange: &str) -> GetBalancesResponse {
        let balance_dtos: Vec<BalanceDto> = balances.into_iter()
            .map(|balance| BalanceDto {
                asset: balance.asset,
                free: balance.free,
                locked: balance.locked,
                total: balance.total,
                usd_value: Decimal::ZERO, // Would be calculated using current prices
            })
            .collect();

        let total_value_usd = balance_dtos.iter().map(|b| b.usd_value).sum();

        GetBalancesResponse {
            exchange: exchange.to_string(),
            balances: balance_dtos,
            total_value_usd,
            timestamp: Utc::now(),
        }
    }

    /// Get popular trading instruments for an exchange
    fn get_popular_instruments(&self, exchange: &str) -> Vec<InstrumentDto> {
        console_log!("TRADING SERVICE: Getting popular instruments for {}", exchange);

        let instruments = match exchange.to_lowercase().as_str() {
            "binance" => vec![
                ("BTCUSDT", "BTC", "USDT"),
                ("ETHUSDT", "ETH", "USDT"),
                ("BNBUSDT", "BNB", "USDT"),
                ("ADAUSDT", "ADA", "USDT"),
                ("DOTUSDT", "DOT", "USDT"),
                ("LINKUSDT", "LINK", "USDT"),
                ("LTCUSDT", "LTC", "USDT"),
                ("BCHUSDT", "BCH", "USDT"),
                ("XLMUSDT", "XLM", "USDT"),
                ("EOSUSDT", "EOS", "USDT"),
            ],
            "coinbase" => vec![
                ("BTC-USD", "BTC", "USD"),
                ("ETH-USD", "ETH", "USD"),
                ("LTC-USD", "LTC", "USD"),
                ("BCH-USD", "BCH", "USD"),
                ("ADA-USD", "ADA", "USD"),
                ("DOT-USD", "DOT", "USD"),
                ("LINK-USD", "LINK", "USD"),
                ("XLM-USD", "XLM", "USD"),
                ("EOS-USD", "EOS", "USD"),
                ("ATOM-USD", "ATOM", "USD"),
            ],
            _ => vec![
                ("BTCUSDT", "BTC", "USDT"),
                ("ETHUSDT", "ETH", "USDT"),
                ("ADAUSDT", "ADA", "USDT"),
                ("DOTUSDT", "DOT", "USDT"),
                ("LINKUSDT", "LINK", "USDT"),
            ],
        };

        instruments.into_iter()
            .map(|(symbol, base, quote)| InstrumentDto {
                symbol: symbol.to_string(),
                base_asset: base.to_string(),
                quote_asset: quote.to_string(),
                exchange: exchange.to_string(),
                instrument_type: "SPOT".to_string(),
                status: "ACTIVE".to_string(),
                min_quantity: Decimal::new(1, 8), // 0.00000001
                max_quantity: Decimal::new(1000000, 0), // 1,000,000
                quantity_precision: 8,
                price_precision: 8,
            })
            .collect()
    }
}

impl Default for TradingService {
    fn default() -> Self {
        Self::new()
    }
}
