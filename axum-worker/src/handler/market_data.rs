use worker::{Request, Response, RouteContext, Result};
use worker::console_log;

use crate::state::AppState;
use crate::dto::market_data::{
    MarketDataSubscriptionRequest, GetInstrumentsRequest, GetTradesRequest, GetTradesResponse
};

/// Handle market data subscription requests (barter-rs inspired)
pub async fn handle_subscribe_market_data(mut req: Request, ctx: RouteContext<AppState>) -> Result<Response> {
    console_log!("MARKET DATA: Handling subscription request (barter-rs style)");

    let request: MarketDataSubscriptionRequest = match req.json::<MarketDataSubscriptionRequest>().await {
        Ok(req) => {
            console_log!("MARKET DATA: Successfully parsed subscription request for {}/{} on {} (barter-rs inspired)",
                req.base, req.quote, req.exchange);
            req
        }
        Err(e) => {
            console_log!("MARKET DATA: Failed to parse JSON request: {}", e);
            return Response::error("Invalid JSON request", 400);
        }
    };

    match ctx.data.market_data_service.subscribe(request).await {
        Ok(response) => {
            console_log!("MARKET DATA: Subscription successful (barter-rs style)");
            Response::from_json(&response)
        }
        Err(e) => {
            console_log!("MARKET DATA: Subscription failed: {}", e);
            Response::error(&format!("Subscription failed: {}", e), 500)
        }
    }
}

/// Handle requests to get available instruments
pub async fn handle_get_instruments(mut req: Request, ctx: RouteContext<AppState>) -> Result<Response> {
    console_log!("MARKET DATA: Handling get instruments request");
    
    let request: GetInstrumentsRequest = match req.json().await {
        Ok(req) => req,
        Err(_) => {
            // If JSON parsing fails, assume no filter (get all instruments)
            GetInstrumentsRequest { exchange: None }
        }
    };
    
    match ctx.data.market_data_service.get_instruments(request.exchange).await {
        Ok(response) => {
            console_log!("MARKET DATA: Found {} instruments", response.instruments.len());
            Response::from_json(&response)
        }
        Err(e) => {
            console_log!("MARKET DATA: Failed to get instruments: {}", e);
            Response::error(&format!("Failed to get instruments: {}", e), 500)
        }
    }
}

/// Handle requests to get recent trades (mock implementation)
pub async fn handle_get_trades(mut req: Request, _ctx: RouteContext<AppState>) -> Result<Response> {
    console_log!("MARKET DATA: Handling get trades request");
    
    let _request: GetTradesRequest = match req.json::<GetTradesRequest>().await {
        Ok(req) => {
            console_log!("MARKET DATA: Getting trades for {}/{} on {}", 
                req.base, req.quote, req.exchange);
            req
        }
        Err(e) => {
            console_log!("MARKET DATA: Failed to parse JSON request: {}", e);
            return Response::error("Invalid JSON request", 400);
        }
    };
    
    // For now, return empty trades list as this is a basic integration
    let response = GetTradesResponse {
        trades: vec![],
    };
    
    console_log!("MARKET DATA: Returning {} trades", response.trades.len());
    Response::from_json(&response)
}

/// Handle market data service status requests
pub async fn handle_market_data_status(_req: Request, ctx: RouteContext<AppState>) -> Result<Response> {
    console_log!("MARKET DATA: Handling status request");
    
    let subscriptions_count = ctx.data.market_data_service.get_active_subscriptions_count().await;
    let is_ready = ctx.data.market_data_service.is_ready().await;
    
    let status = serde_json::json!({
        "status": if is_ready { "ready" } else { "initializing" },
        "active_subscriptions": subscriptions_count,
        "service": "market_data",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "barter_integration": "basic"
    });
    
    console_log!("MARKET DATA: Status - Ready: {}, Subscriptions: {}", is_ready, subscriptions_count);
    Response::from_json(&status)
}
