#![allow(dead_code)]
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

/// WASM-compatible HTTP client for making API requests
#[derive(Clone)]
pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    /// Create a new API client instance
    pub fn new() -> Self {
        let client = Client::builder()
            .build()
            .expect("Failed to create HTTP client");

        // Frontend runs on 3000, backend runs on 8787
        // API requests must go to backend server, not frontend
        let base_url = "http://127.0.0.1:8787/api".to_string();

        Self { client, base_url }
    }

    /// Create API client with custom base URL
    pub fn with_base_url(base_url: impl Into<String>) -> Self {
        let client = Client::builder()
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url: base_url.into(),
        }
    }

    /// Make a GET request
    pub async fn get<T>(&self, endpoint: &str) -> Result<T, String>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        
        let response = self.client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("GET request failed: {}", e))?;

        if response.status().is_success() {
            response
                .json::<T>()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))
        } else {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(format!("Request failed with status {}: {}", status, error_text))
        }
    }

    /// Make a POST request with JSON body
    pub async fn post<T, R>(&self, endpoint: &str, data: &T) -> Result<R, String>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        
        let response = self.client
            .post(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .json(data)
            .send()
            .await
            .map_err(|e| format!("POST request failed: {}", e))?;

        if response.status().is_success() {
            response
                .json::<R>()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))
        } else {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(format!("Request failed with status {}: {}", status, error_text))
        }
    }

    /// Make a POST request without body (for logout, etc.)
    pub async fn post_empty<R>(&self, endpoint: &str) -> Result<R, String>
    where
        R: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        
        let response = self.client
            .post(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("POST request failed: {}", e))?;

        if response.status().is_success() {
            response
                .json::<R>()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))
        } else {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(format!("Request failed with status {}: {}", status, error_text))
        }
    }

    /// Make a PUT request with JSON body
    pub async fn put<T, R>(&self, endpoint: &str, data: &T) -> Result<R, String>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        
        let response = self.client
            .put(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .json(data)
            .send()
            .await
            .map_err(|e| format!("PUT request failed: {}", e))?;

        if response.status().is_success() {
            response
                .json::<R>()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))
        } else {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(format!("Request failed with status {}: {}", status, error_text))
        }
    }

    /// Make a DELETE request
    pub async fn delete<R>(&self, endpoint: &str) -> Result<R, String>
    where
        R: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        
        let response = self.client
            .delete(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| format!("DELETE request failed: {}", e))?;

        if response.status().is_success() {
            response
                .json::<R>()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))
        } else {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(format!("Request failed with status {}: {}", status, error_text))
        }
    }

    /// Make a request with custom headers
    pub async fn request_with_headers<T, R>(
        &self,
        method: reqwest::Method,
        endpoint: &str,
        data: Option<&T>,
        headers: HashMap<String, String>,
    ) -> Result<R, String>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        
        let mut request = self.client
            .request(method, &url)
            .header("Accept", "application/json");

        // Add custom headers
        for (key, value) in headers {
            request = request.header(&key, &value);
        }

        // Add JSON body if provided
        if let Some(data) = data {
            request = request
                .header("Content-Type", "application/json")
                .json(data);
        }

        let response = request
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if response.status().is_success() {
            response
                .json::<R>()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))
        } else {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(format!("Request failed with status {}: {}", status, error_text))
        }
    }
}

impl Default for ApiClient {
    fn default() -> Self {
        Self::new()
    }
}

/// Check if the browser is online (simplified implementation)
pub fn is_online() -> bool {
    if let Some(window) = web_sys::window() {
        let navigator = window.navigator();
        navigator.on_line()
    } else {
        true // Assume online if no window
    }
}

/// Log messages to browser console (useful for debugging WASM)
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/// Macro for logging to browser console
#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (crate::services::api_client::log(&format_args!($($t)*).to_string()))
}

#[allow(unused_imports)]
pub use console_log;