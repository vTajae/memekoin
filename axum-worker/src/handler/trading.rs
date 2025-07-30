use worker::{Request, Response, RouteContext, Result};
use worker::console_log;

use crate::state::AppState;
use crate::dto::trading::{
    GetQuoteRequest, GetOrderBookRequest, PlaceOrderRequest, GetBalancesRequest,
    GetInstrumentsRequest, GetTradingStatusRequest, TradingErrorResponse
};

/// Helper function to create error responses
fn create_error_response(error: &TradingErrorResponse) -> Result<Response> {
    match Response::from_json(error) {
        Ok(resp) => Ok(resp.with_status(500)),
        Err(_) => Response::error("Internal server error", 500),
    }
}

/// Handle market quote requests
pub async fn handle_get_quote(mut req: Request, ctx: RouteContext<AppState>) -> Result<Response> {
    console_log!("TRADING HANDLER: Handling get quote request");
    
    let request: GetQuoteRequest = match req.json::<GetQuoteRequest>().await {
        Ok(req) => {
            console_log!("TRADING HANDLER: Successfully parsed quote request for {} on {}", 
                req.symbol, req.exchange);
            req
        }
        Err(e) => {
            console_log!("TRADING HANDLER: Failed to parse JSON request: {}", e);
            return Response::error("Invalid JSON request", 400);
        }
    };
    
    match ctx.data.trading_service.get_quote(request).await {
        Ok(response) => {
            console_log!("TRADING HANDLER: Successfully retrieved quote");
            Response::from_json(&response)
        }
        Err(e) => {
            console_log!("TRADING HANDLER: Failed to get quote: {}", e.error);
            create_error_response(&e)
        }
    }
}

/// Handle order book requests
pub async fn handle_get_order_book(mut req: Request, ctx: RouteContext<AppState>) -> Result<Response> {
    console_log!("TRADING HANDLER: Handling get order book request");
    
    let request: GetOrderBookRequest = match req.json::<GetOrderBookRequest>().await {
        Ok(req) => {
            console_log!("TRADING HANDLER: Successfully parsed order book request for {} on {}", 
                req.symbol, req.exchange);
            req
        }
        Err(e) => {
            console_log!("TRADING HANDLER: Failed to parse JSON request: {}", e);
            return Response::error("Invalid JSON request", 400);
        }
    };
    
    match ctx.data.trading_service.get_order_book(request).await {
        Ok(response) => {
            console_log!("TRADING HANDLER: Successfully retrieved order book");
            Response::from_json(&response)
        }
        Err(e) => {
            console_log!("TRADING HANDLER: Failed to get order book: {}", e.error);
            create_error_response(&e)
        }
    }
}

/// Handle order placement requests
pub async fn handle_place_order(mut req: Request, ctx: RouteContext<AppState>) -> Result<Response> {
    console_log!("TRADING HANDLER: Handling place order request");
    
    let request: PlaceOrderRequest = match req.json::<PlaceOrderRequest>().await {
        Ok(req) => {
            console_log!("TRADING HANDLER: Successfully parsed place order request for {} {} {} on {}", 
                req.side, req.quantity, req.symbol, req.exchange);
            req
        }
        Err(e) => {
            console_log!("TRADING HANDLER: Failed to parse JSON request: {}", e);
            return Response::error("Invalid JSON request", 400);
        }
    };
    
    match ctx.data.trading_service.place_order(request).await {
        Ok(response) => {
            console_log!("TRADING HANDLER: Successfully placed order");
            Response::from_json(&response)
        }
        Err(e) => {
            console_log!("TRADING HANDLER: Failed to place order: {}", e.error);
            create_error_response(&e)
        }
    }
}

/// Handle balance requests
pub async fn handle_get_balances(mut req: Request, ctx: RouteContext<AppState>) -> Result<Response> {
    console_log!("TRADING HANDLER: Handling get balances request");
    
    let request: GetBalancesRequest = match req.json::<GetBalancesRequest>().await {
        Ok(req) => {
            console_log!("TRADING HANDLER: Successfully parsed balances request for {}", req.exchange);
            req
        }
        Err(e) => {
            console_log!("TRADING HANDLER: Failed to parse JSON request: {}", e);
            return Response::error("Invalid JSON request", 400);
        }
    };
    
    match ctx.data.trading_service.get_balances(request).await {
        Ok(response) => {
            console_log!("TRADING HANDLER: Successfully retrieved balances");
            Response::from_json(&response)
        }
        Err(e) => {
            console_log!("TRADING HANDLER: Failed to get balances: {}", e.error);
            create_error_response(&e)
        }
    }
}

/// Handle instruments requests
pub async fn handle_get_instruments(mut req: Request, ctx: RouteContext<AppState>) -> Result<Response> {
    console_log!("TRADING HANDLER: Handling get instruments request");
    
    let request: GetInstrumentsRequest = match req.json::<GetInstrumentsRequest>().await {
        Ok(req) => {
            console_log!("TRADING HANDLER: Successfully parsed instruments request for {}", req.exchange);
            req
        }
        Err(e) => {
            console_log!("TRADING HANDLER: Failed to parse JSON request: {}", e);
            return Response::error("Invalid JSON request", 400);
        }
    };
    
    match ctx.data.trading_service.get_instruments(request).await {
        Ok(response) => {
            console_log!("TRADING HANDLER: Successfully retrieved {} instruments", response.instruments.len());
            Response::from_json(&response)
        }
        Err(e) => {
            console_log!("TRADING HANDLER: Failed to get instruments: {}", e.error);
            create_error_response(&e)
        }
    }
}

/// Handle trading status requests
pub async fn handle_get_trading_status(mut req: Request, ctx: RouteContext<AppState>) -> Result<Response> {
    console_log!("TRADING HANDLER: Handling get trading status request");
    
    let request: GetTradingStatusRequest = match req.json::<GetTradingStatusRequest>().await {
        Ok(req) => {
            console_log!("TRADING HANDLER: Successfully parsed trading status request for {}", req.exchange);
            req
        }
        Err(e) => {
            console_log!("TRADING HANDLER: Failed to parse JSON request: {}", e);
            return Response::error("Invalid JSON request", 400);
        }
    };
    
    match ctx.data.trading_service.get_trading_status(request).await {
        Ok(response) => {
            console_log!("TRADING HANDLER: Successfully retrieved trading status");
            Response::from_json(&response)
        }
        Err(e) => {
            console_log!("TRADING HANDLER: Failed to get trading status: {}", e.error);
            create_error_response(&e)
        }
    }
}

/// Handle trading service health check
pub async fn handle_trading_health(_req: Request, _ctx: RouteContext<AppState>) -> Result<Response> {
    console_log!("TRADING HANDLER: Handling trading health check");
    
    let health_status = serde_json::json!({
        "status": "healthy",
        "service": "trading",
        "barter_integration": "active",
        "supported_exchanges": ["binance", "coinbase", "kraken", "okx", "bybit"],
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "version": "1.0.0"
    });
    
    console_log!("TRADING HANDLER: Trading service health check completed");
    Response::from_json(&health_status)
}

/// Handle trading service configuration
pub async fn handle_trading_config(_req: Request, _ctx: RouteContext<AppState>) -> Result<Response> {
    console_log!("TRADING HANDLER: Handling trading configuration request");
    
    let config = serde_json::json!({
        "supported_exchanges": [
            {
                "name": "binance",
                "display_name": "Binance",
                "supported_features": ["spot", "futures", "margin"],
                "rate_limits": {
                    "requests_per_second": 10,
                    "orders_per_second": 5
                }
            },
            {
                "name": "coinbase",
                "display_name": "Coinbase Pro",
                "supported_features": ["spot"],
                "rate_limits": {
                    "requests_per_second": 10,
                    "orders_per_second": 5
                }
            },
            {
                "name": "kraken",
                "display_name": "Kraken",
                "supported_features": ["spot", "futures"],
                "rate_limits": {
                    "requests_per_second": 1,
                    "orders_per_second": 1
                }
            }
        ],
        "order_types": ["MARKET", "LIMIT", "STOP_LOSS", "TAKE_PROFIT"],
        "time_in_force": ["GTC", "IOC", "FOK"],
        "default_precision": {
            "price": 8,
            "quantity": 8
        }
    });
    
    console_log!("TRADING HANDLER: Trading configuration retrieved");
    Response::from_json(&config)
}
