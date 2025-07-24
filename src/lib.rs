#[cfg(feature = "ssr")]
use worker::*;

#[cfg(feature = "ssr")]
use crate::app::{App, shell};

#[cfg(feature = "hydrate")]
use crate::app::App;

pub mod app;
mod components;

#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    // use leptos::server_fn::axum::register_explicit;
    // Add all of your server functions here when you have them
    // register_explicit::<YourServerFunction>();
}

#[cfg(feature = "ssr")]
async fn router(env: Env) -> axum::Router {
    use std::sync::Arc;
    use axum::{Extension, Router};
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    register_server_functions();

    // build our application with a route
    Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .with_state(leptos_options)
        .layer(Extension(Arc::new(env))) // <- Allow leptos server functions to access Worker stuff
}

#[cfg(feature = "ssr")]
#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> Result<http::Response<axum::body::Body>> {
    use tower_service::Service;

    console_error_panic_hook::set_once();

    Ok(router(env).await.call(req).await?)
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}






