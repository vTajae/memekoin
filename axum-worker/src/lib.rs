// Following rusty-worker architecture pattern
pub mod dto;
pub mod util;
pub mod repo;
pub mod entity;
pub mod handler;
pub mod service;
pub mod router;
pub mod state;
pub mod clients;

use worker::{Router, *};
use crate::state::AppState;

#[event(start)]
fn start() {
    console_error_panic_hook::set_once();
}

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    // Get environment variables
    let jwt_secret = env.secret("JWT_SECRET")?.to_string();
    let db_connection_string = env.secret("DB_CONNECTION_STRING")?.to_string();

    // Initialize app state with LIVE Neon database connection
    console_log!("Initializing rusty-worker with LIVE Neon database");

    // Create repository with LIVE database connection
    let user_repository = crate::repo::user::UserRepository::new(db_connection_string);
    let auth_service = crate::service::auth::AuthenticationService::new(jwt_secret);
    let market_data_service = crate::service::market_data::MarketDataService::new();
    let trading_service = crate::service::trading::TradingService::new();

    console_log!("Initializing services with barter-rs trading integration");

    let app_state = AppState {
        user_repository,
        auth_service,
        market_data_service,
        trading_service,
    };

    // Create router with app state
    let router = Router::with_data(app_state);
    let configured_router = crate::router::configure_routes(router);

    // Run the request through the router
    configured_router.run(req, env).await
}