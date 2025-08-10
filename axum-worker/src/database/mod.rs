//! Database module - Cloudflare Workers compatible database layer
//! 
//! Uses NeonClient for real PostgreSQL connections via worker::Socket
//! This bypasses the mio limitation by using Cloudflare's network bindings

use crate::{error::AppError, client::neon_client::NeonClient};
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
    client: NeonClient,
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
        Ok(Self { client, config })
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
        Ok(Self { client, config })
    }

    /// Execute a query using NeonClient
    pub async fn query(&self, query: &str, params: &[&(dyn tokio_postgres::types::ToSql + Sync)]) -> Result<Vec<Row>, AppError> {
        self.client.execute_query(query, params).await
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
        let affected_rows = self.client.client.execute(query, params).await
            .map_err(|e| AppError::DatabaseError(format!("Execute failed: {}", e)))?;
        Ok(affected_rows)
    }

    /// Test database connection
    pub async fn test_connection(&self) -> Result<(), AppError> {
        self.client.test_connection().await
    }
}

/// Helper trait for converting database rows to entities  
pub trait FromRow<T> {
    fn from_row(row: Row) -> Result<T, AppError>;
}

