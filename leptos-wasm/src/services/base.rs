use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tracing::{error, info, debug};

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

    /// Make a GET request
    #[tracing::instrument(name = "api_client.get", skip(self), fields(endpoint = %endpoint))]
    pub async fn get<T>(&self, endpoint: &str) -> Result<T, String>
    where
        T: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        info!("📡 GET {}", url);
        
        let response = self.client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| {
                error!("GET {} failed: {}", url, e);
                format!("GET request failed: {}", e)
            })?;

        let status = response.status();
        debug!("Response status: {}", status);

        if response.status().is_success() {
            response
                .json::<T>()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            error!("Request failed: {} - {}", status, error_text);
            Err(format!("Request failed with status {}: {}", status, error_text))
        }
    }

    /// Make a POST request with JSON body
    #[tracing::instrument(name = "api_client.post", skip(self, data), fields(endpoint = %endpoint))]
    pub async fn post<T, R>(&self, endpoint: &str, data: &T) -> Result<R, String>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        info!("📡 POST {}", url);
        
        let response = self.client
            .post(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .json(data)
            .send()
            .await
            .map_err(|e| { 
                error!("POST {} failed: {}", url, e); 
                format!("POST request failed: {}", e) 
            })?;

        let status = response.status();
        debug!("Response status: {}", status);

        if response.status().is_success() {
            response
                .json::<R>()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            error!("Request failed: {} - {}", status, error_text);
            Err(format!("Request failed with status {}: {}", status, error_text))
        }
    }

    /// Make a POST request with JSON body and custom headers
    #[tracing::instrument(name = "api_client.post_with_headers", skip(self, data), fields(endpoint = %endpoint))]
    pub async fn post_with_headers<T, R>(&self, endpoint: &str, data: &T, headers: &[(&str, &str)]) -> Result<R, String>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        info!("📡 POST {} (with headers)", url);
        let mut req = self.client
            .post(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json");
        for (k, v) in headers.iter() {
            req = req.header(*k, *v);
        }
        let response = req
            .json(data)
            .send()
            .await
            .map_err(|e| { 
                error!("POST {} failed: {}", url, e); 
                format!("POST request failed: {}", e) 
            })?;

        let status = response.status();
        debug!("Response status: {}", status);

        if response.status().is_success() {
            response
                .json::<R>()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            error!("Request failed: {} - {}", status, error_text);
            Err(format!("Request failed with status {}: {}", status, error_text))
        }
    }

    /// Make a POST request without body (for logout, etc.)
    #[tracing::instrument(name = "api_client.post_empty", skip(self), fields(endpoint = %endpoint))]
    pub async fn post_empty<R>(&self, endpoint: &str) -> Result<R, String>
    where
        R: DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        info!("📡 POST (empty) {}", url);
        
        let response = self.client
            .post(&url)
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| { 
                error!("POST {} failed: {}", url, e); 
                format!("POST request failed: {}", e) 
            })?;

        let status = response.status();
        debug!("Response status: {}", status);

        if response.status().is_success() {
            response
                .json::<R>()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            error!("Request failed: {} - {}", status, error_text);
            Err(format!("Request failed with status {}: {}", status, error_text))
        }
    }

}

impl Default for ApiClient {
    fn default() -> Self {
        Self::new()
    }
}