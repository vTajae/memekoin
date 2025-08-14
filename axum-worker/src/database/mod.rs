//! Database module - Cloudflare Workers compatible database layer
//! 
//! Uses NeonClient for real PostgreSQL connections via worker::Socket
//! This bypasses the mio limitation by using Cloudflare's network bindings

use crate::{utils::error::AppError, client::neon_client::NeonClient};
use worker::Env;

pub mod test_connection;

/// Database configuration for Cloudflare Workers with NeonClient
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub connection_string: String,
    pub max_retries: u32,
    pub retry_delay_ms: u64,
    pub query_timeout_ms: u64,
}

impl DatabaseConfig {
    pub fn from_env(connection_string: String) -> Self {
        Self {
            connection_string,
            max_retries: 3,
            retry_delay_ms: 1000,
            query_timeout_ms: 30000,
        }
    }
}

/// WASM-compatible database row representation using tokio-postgres with "js" feature
pub type Row = tokio_postgres::Row;

/// WASM-compatible database using NeonClient
#[derive(Clone)]
pub struct Database {
    client: Option<NeonClient>,
    #[allow(dead_code)]
    config: DatabaseConfig,
}

impl Database {
    /// Create new database with NeonClient from environment
    pub async fn from_env(env: &Env) -> Result<Self, AppError> {
        worker::console_log!("DATABASE::from_env - Starting database initialization");
        
        // Test environment variable access first
        let connection_string = env.secret("DEV_DATABASE_URL")
            .map(|s| s.to_string())
            .or_else(|_| env.var("DEV_DATABASE_URL").map(|v| v.to_string()))
            .map_err(|_| {
                worker::console_log!("DATABASE::from_env - DEV_DATABASE_URL not found in environment");
                AppError::DatabaseError("DEV_DATABASE_URL not found".to_string())
            })?;
            
        worker::console_log!("DATABASE::from_env - Environment variable found: {}...", 
            connection_string.chars().take(30).collect::<String>());
        
        // Now try to create NeonClient using the connection string we already have
        worker::console_log!("DATABASE::from_env - Creating NeonClient with connection string");
    let client = NeonClient::new(connection_string.clone()).await.map_err(|e| {
            worker::console_log!("DATABASE::from_env - NeonClient creation failed: {:?}", e);
            e
        })?;
        
        worker::console_log!("DATABASE::from_env - NeonClient created successfully");
        let config = DatabaseConfig::from_env(connection_string);
        
        worker::console_log!("DATABASE::from_env - Database initialization complete");
    Ok(Self { client: Some(client), config })
    }

    /// Create new database with an explicit connection string (preferred when AppState resolves env)
    pub async fn from_url(connection_string: &str) -> Result<Self, AppError> {
        let connection_string = connection_string.to_string();
        if connection_string.is_empty() {
            return Err(AppError::DatabaseError("Empty database URL".to_string()));
        }

        worker::console_log!(
            "DATABASE::from_url - Creating NeonClient with URL: {}...",
            connection_string.chars().take(30).collect::<String>()
        );

        let client = NeonClient::new(connection_string.clone()).await?;
        let config = DatabaseConfig::from_env(connection_string);
        Ok(Self { client: Some(client), config })
    }

    /// Lenient initializer: returns a stub Database when connection fails.
    /// Useful for local dev where DB might be unavailable.
    pub async fn from_url_or_stub(connection_string: &str) -> Self {
        let connection_string = connection_string.to_string();
        worker::console_log!(
            "DATABASE::from_url_or_stub - Attempting NeonClient with URL: {}...",
            connection_string.chars().take(30).collect::<String>()
        );
        match NeonClient::new(connection_string.clone()).await {
            Ok(client) => {
                worker::console_log!("DATABASE::from_url_or_stub - Connected successfully");
                Self { client: Some(client), config: DatabaseConfig::from_env(connection_string) }
            }
            Err(e) => {
                worker::console_log!("DATABASE::from_url_or_stub - Connection failed, continuing without DB: {:?}", e);
                Self { client: None, config: DatabaseConfig::from_env(connection_string) }
            }
        }
    }

    /// Execute a query using NeonClient
    pub async fn query(&self, query: &str, params: &[&(dyn tokio_postgres::types::ToSql + Sync)]) -> Result<Vec<Row>, AppError> {
        match &self.client {
            Some(c) => c.execute_query(query, params).await,
            None => Err(AppError::DatabaseError("Database not initialized".to_string())),
        }
    }

    /// Execute a query expecting a single row
    pub async fn query_one(&self, query: &str, params: &[&(dyn tokio_postgres::types::ToSql + Sync)]) -> Result<Row, AppError> {
        let rows = self.query(query, params).await?;
        rows.into_iter().next()
            .ok_or_else(|| AppError::DatabaseError("Expected one row, got none".to_string()))
    }

    /// Execute a query expecting an optional row
    pub async fn query_opt(&self, query: &str, params: &[&(dyn tokio_postgres::types::ToSql + Sync)]) -> Result<Option<Row>, AppError> {
        let rows = self.query(query, params).await?;
        Ok(rows.into_iter().next())
    }

    /// Execute a statement (INSERT, UPDATE, DELETE)
    pub async fn execute(&self, query: &str, params: &[&(dyn tokio_postgres::types::ToSql + Sync)]) -> Result<u64, AppError> {
        match &self.client {
            Some(c) => {
                let affected_rows = c.client.execute(query, params).await
                    .map_err(|e| AppError::DatabaseError(format!("Execute failed: {}", e)))?;
                Ok(affected_rows)
            }
            None => Err(AppError::DatabaseError("Database not initialized".to_string())),
        }
    }

    /// Test database connection
    pub async fn test_connection(&self) -> Result<(), AppError> {
        match &self.client {
            Some(c) => c.test_connection().await,
            None => Err(AppError::DatabaseError("Database not initialized".to_string())),
        }
    }
}

/// Helper trait for converting database rows to entities  
pub trait FromRow<T> {
    fn from_row(row: Row) -> Result<T, AppError>;
}

