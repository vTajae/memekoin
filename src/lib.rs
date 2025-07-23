#[cfg(feature = "ssr")]
use worker::*;

pub mod app;
pub mod components;
pub mod pages;
pub mod server;
pub mod utils;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();

    // Notify that WASM has loaded for performance tracking
    if let Some(window) = web_sys::window() {
        let _ = window.dispatch_event(&web_sys::CustomEvent::new("wasmLoaded").unwrap());
    }

    // Use hydrate_body to hydrate the existing server-rendered content
    leptos::mount::hydrate_body(crate::app::App);
}

#[cfg(feature = "ssr")]
#[worker::event(fetch)]
async fn fetch(
    req: worker::HttpRequest,
    _env: worker::Env,
    _ctx: worker::Context,
) -> worker::Result<axum::http::Response<axum::body::Body>> {
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use tower_service::Service;

    console_error_panic_hook::set_once();

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(crate::app::App);

    // build our application with a route
    let router = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || view! { <crate::app::Shell _options=leptos_options.clone() /> }
        })
        .with_state(leptos_options);

    router.call(req).await.map_err(|_| worker::Error::RustError("Router error".to_string()))
}
