//! CORS Middleware - Cross-Origin Resource Sharing configuration

use axum::{
    extract::Request,
    http::{HeaderValue, Method},
    response::Response,
};
use tower::{Layer, Service};
use std::task::{Context, Poll};

/// CORS layer for handling cross-origin requests
#[derive(Clone)]
pub struct CorsLayer {
    allow_origins: Vec<String>,
    allow_methods: Vec<Method>,
    allow_headers: Vec<String>,
    allow_credentials: bool,
}

impl Default for CorsLayer {
    fn default() -> Self {
        Self {
            allow_origins: vec![
                "http://localhost:8787".to_string(),
                "http://127.0.0.1:3000".to_string(),
                "http://localhost:3000".to_string(),
                "https://your-domain.com".to_string(),
            ],
            allow_methods: vec![
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::DELETE,
                Method::OPTIONS,
            ],
            allow_headers: vec![
                "content-type".to_string(),
                "authorization".to_string(),
                "x-requested-with".to_string(),
            ],
            allow_credentials: true,
        }
    }
}

impl CorsLayer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn allow_origin(mut self, origin: &str) -> Self {
        self.allow_origins.push(origin.to_string());
        self
    }

    pub fn allow_method(mut self, method: Method) -> Self {
        self.allow_methods.push(method);
        self
    }

    pub fn allow_header(mut self, header: &str) -> Self {
        self.allow_headers.push(header.to_string());
        self
    }

    pub fn allow_credentials(mut self, allow: bool) -> Self {
        self.allow_credentials = allow;
        self
    }
}

impl<S> Layer<S> for CorsLayer {
    type Service = CorsService<S>;

    fn layer(&self, service: S) -> Self::Service {
        CorsService {
            inner: service,
            cors_config: self.clone(),
        }
    }
}

#[derive(Clone)]
pub struct CorsService<S> {
    inner: S,
    cors_config: CorsLayer,
}

impl<S> Service<Request> for CorsService<S>
where
    S: Service<Request, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let cors_config = self.cors_config.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            // Handle preflight OPTIONS requests
            if request.method() == Method::OPTIONS {
                let mut response = Response::builder()
                    .status(200)
                    .body(axum::body::Body::empty())
                    .unwrap();

                add_cors_headers(&mut response, &cors_config, request.headers().get("origin"));
                return Ok(response);
            }

            // Process the request
            let mut response = inner.call(request).await?;

            // Add CORS headers to the response
            add_cors_headers(&mut response, &cors_config, None);

            Ok(response)
        })
    }
}

fn add_cors_headers(response: &mut Response, cors_config: &CorsLayer, origin: Option<&HeaderValue>) {
    let headers = response.headers_mut();

    // Set Access-Control-Allow-Origin
    if let Some(origin_header) = origin {
        if let Ok(origin_str) = origin_header.to_str() {
            if cors_config.allow_origins.contains(&origin_str.to_string()) 
                || cors_config.allow_origins.contains(&"*".to_string()) {
                headers.insert("access-control-allow-origin", origin_header.clone());
            }
        }
    } else {
        // For non-preflight requests, use the first allowed origin or *
        let origin_value = cors_config.allow_origins.first()
            .map(|o| o.as_str())
            .unwrap_or("*");
        headers.insert("access-control-allow-origin", HeaderValue::from_str(origin_value).unwrap());
    }

    // Set other CORS headers
    let methods = cors_config.allow_methods.iter()
        .map(|m| m.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    headers.insert("access-control-allow-methods", HeaderValue::from_str(&methods).unwrap());

    let headers_str = cors_config.allow_headers.join(", ");
    headers.insert("access-control-allow-headers", HeaderValue::from_str(&headers_str).unwrap());

    if cors_config.allow_credentials {
        headers.insert("access-control-allow-credentials", HeaderValue::from_static("true"));
    }

    headers.insert("access-control-max-age", HeaderValue::from_static("86400"));
}