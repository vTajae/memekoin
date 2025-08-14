#![allow(dead_code)]
//! Axum Worker Backend - Trading Dashboard API
//! 
//! High-performance backend server built with Leptos + Axum for Cloudflare Workers.
//! Provides REST API endpoints for trading data, user management, and system monitoring.

use tower_service::Service;
use worker::*;

pub mod client;
pub mod database;
pub mod dto;
pub mod entity;
pub mod utils;
pub mod handler;
pub mod middleware;
pub mod repository;
pub mod routes;
pub mod service;
pub mod state;

// use tracing_subscriber::{fmt::format::Pretty, prelude::*}; // Unused (tracing init disabled)
// use tracing_web::{performance_layer, MakeConsoleWriter};   // Unused (tracing init disabled)

use crate::{
    middleware::session::simple_session_setup,
    routes::create_router,
    state::AppState,
};


#[event(start)]
fn start() {
    // let fmt_layer = tracing_subscriber::fmt::layer()
    //     .json()
    //     .with_ansi(false) // Only partially supported across JavaScript runtimes
    //     .without_time() // Disable timestamps - SystemTime not available in WASM
    //     .with_writer(MakeConsoleWriter); // write events to the console

    // let perf_layer = performance_layer().with_details_from_fields(Pretty::default());

    // tracing_subscriber::registry()
    //     .with(fmt_layer)
    //     .with(perf_layer)
    //     .init();
}


#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();
    
    // Initialize application state with environment variables
    let app_state = AppState::new(env).await
        .map_err(|e| worker::Error::RustError(format!("Failed to initialize app state: {}", e)))?;
    


    // Initialize simple cookie-based session management
    simple_session_setup(&app_state).await
        .map_err(|e| worker::Error::RustError(format!("Failed to setup sessions: {}", e)))?;

    // Create router without complex session middleware - using simple cookies
    let mut router = create_router()
        .await
        .map_err(|e| worker::Error::RustError(format!("Failed to create router: {}", e)))?
        .with_state(app_state.clone());

    Ok(router.call(req).await?)
}
