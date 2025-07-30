use crate::repo::user::UserRepository;
use crate::service::auth::AuthenticationService;
use crate::service::market_data::MarketDataService;
use crate::service::trading::TradingService;
use worker::console_log;

/// Application state following rusty-worker pattern
#[derive(Clone)]
pub struct AppState {
    pub user_repository: UserRepository,
    pub auth_service: AuthenticationService,
    pub market_data_service: MarketDataService,
    pub trading_service: TradingService,
}

/// Initialize the application state with LIVE Neon database integration
pub async fn init_app_state(database_url: String, jwt_secret: String) -> Result<AppState, String> {
    console_log!("Initializing application state with LIVE Neon database connection");
    console_log!("Database URL: {}", &database_url[..50]); // Show first 50 chars for verification

    // Create user repository with LIVE Neon database connection
    let user_repository = UserRepository::new(database_url);
    let auth_service = AuthenticationService::new(jwt_secret);
    let market_data_service = MarketDataService::new();
    let trading_service = TradingService::new();

    console_log!("Application state initialized successfully with LIVE Neon database, market data service, and trading service");
    Ok(AppState {
        user_repository,
        auth_service,
        market_data_service,
        trading_service,
    })
}
