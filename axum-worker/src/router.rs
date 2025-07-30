use worker::*;
use crate::state::AppState;
use crate::handler::auth::{handle_register, handle_login};
use crate::handler::market_data::{
    handle_subscribe_market_data, handle_get_instruments,
    handle_get_trades, handle_market_data_status
};
use crate::handler::trading::{
    handle_get_quote, handle_get_order_book, handle_place_order,
    handle_get_balances, handle_get_instruments as handle_get_trading_instruments,
    handle_get_trading_status, handle_trading_health, handle_trading_config
};

pub fn configure_routes(router: Router<AppState>) -> Router<AppState> {
    router
        .get("/", |_req, _ctx| Response::ok("Hello from Rusty Worker with Barter-rs Trading!"))
        .get("/api/health", |_req, _ctx| Response::ok("OK"))
        // Authentication routes
        .post_async("/api/auth/register", handle_register)
        .post_async("/api/auth/login", handle_login)
        // Market data routes
        .post_async("/api/market-data/subscribe", handle_subscribe_market_data)
        .post_async("/api/market-data/instruments", handle_get_instruments)
        .post_async("/api/market-data/trades", handle_get_trades)
        .get_async("/api/market-data/status", handle_market_data_status)
        // Trading routes - barter-rs integration
        .post_async("/api/trading/quote", handle_get_quote)
        .post_async("/api/trading/orderbook", handle_get_order_book)
        .post_async("/api/trading/order", handle_place_order)
        .post_async("/api/trading/balances", handle_get_balances)
        .post_async("/api/trading/instruments", handle_get_trading_instruments)
        .post_async("/api/trading/status", handle_get_trading_status)
        .get_async("/api/trading/health", handle_trading_health)
        .get_async("/api/trading/config", handle_trading_config)
}
