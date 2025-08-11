use worker::{console_log, Env, Socket, postgres_tls::PassthroughTls};
use crate::utils::error::AppError;
use tokio_postgres::{Client, Row, config::Host};
use std::sync::Arc;
use std::str::FromStr;


/// Client for Neon PostgreSQL database operations (WASM-compatible via worker::Socket + "js" feature)
#[derive(Clone)]
pub struct NeonClient {
    pub(crate) client: Arc<Client>,
}

impl NeonClient {
    /// Initialize NeonClient from environment variables with WASM-compatible TLS handling
    pub async fn from_env(env: &Env) -> Result<Self, AppError> {
        console_log!("NEON_CLIENT::from_env - Starting initialization");
        
        // Try secret first, then fallback to variable
        let database_url = env.secret("DEV_DATABASE_URL")
            .map(|s| s.to_string())
            .or_else(|_| {
                console_log!("NEON_CLIENT::from_env - Secret not found, trying var");
                env.var("DEV_DATABASE_URL").map(|v| v.to_string())
            })
            .map_err(|e| {
                console_log!("NEON_CLIENT::from_env - Environment variable access failed: {:?}", e);
                AppError::DatabaseError("DEV_DATABASE_URL not found in environment".to_string())
            })?;

        console_log!("NEON_CLIENT: Database URL found: {}", 
            format!("{}...", database_url.chars().take(30).collect::<String>()));

        // Use the exact pattern from the working reference
        let config = tokio_postgres::Config::from_str(&database_url)
            .map_err(|e| AppError::DatabaseError(format!("Failed to parse database URL: {}", e)))?;

        let host = match &config.get_hosts()[0] {
            Host::Tcp(host) => host,
        };

        let port: u16 = config.get_ports()[0];

        console_log!("NEON_CLIENT: Connecting to {}:{}", host, port);

        let socket = Socket::builder()
            .secure_transport(worker::SecureTransport::StartTls)
            .connect(host, port)
            .map_err(|e| AppError::DatabaseError(format!("Failed to create socket: {:?}", e)))?;

        let (client, connection) = config
            .connect_raw(socket, PassthroughTls)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to connect: {:?}", e)))?;

        // Spawn connection handler
        wasm_bindgen_futures::spawn_local(async move {
            if let Err(e) = connection.await {
                console_log!("NEON_CLIENT: Connection error: {}", e);
            }
        });

        console_log!("NEON_CLIENT: Successfully connected to Neon PostgreSQL database");
        
        Ok(Self {
            client: Arc::new(client),
        })
    }

    /// Create mock NeonClient for development (doesn't connect to real database)
    pub async fn new_mock() -> Result<Self, AppError> {
        console_log!("NEON_CLIENT: Creating mock client for development");
        
        // Create a mock client that doesn't actually connect to anything
        // This is a temporary solution for development
        
        // For now, we'll create a placeholder that will fail gracefully
        // In a real implementation, this would be a proper mock
        Err(AppError::DatabaseError("Mock database not fully implemented - Phase 2 OAuth will work without database persistence".to_string()))
    }

    /// Create NeonClient with explicit configuration (for testing)
    pub async fn new(database_url: String) -> Result<Self, AppError> {
        console_log!("NEON_CLIENT: Creating new client with URL: {}", 
            format!("{}...", database_url.chars().take(30).collect::<String>()));
        
        // Use the exact pattern from the working reference
        let config = tokio_postgres::Config::from_str(&database_url)
            .map_err(|e| AppError::DatabaseError(format!("Failed to parse database URL: {}", e)))?;

        let host = match &config.get_hosts()[0] {
            Host::Tcp(host) => host,
        };

        let port: u16 = config.get_ports()[0];

        let socket = Socket::builder()
            .secure_transport(worker::SecureTransport::StartTls)
            .connect(host, port)
            .map_err(|e| AppError::DatabaseError(format!("Failed to create socket: {:?}", e)))?;

        let (client, connection) = config
            .connect_raw(socket, PassthroughTls)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Failed to connect: {:?}", e)))?;

        wasm_bindgen_futures::spawn_local(async move {
            if let Err(e) = connection.await {
                console_log!("NEON_CLIENT: Connection error: {}", e);
            }
        });

        Ok(Self {
            client: Arc::new(client),
        })
    }

    /// Execute SQL query with parameters using tokio-postgres
    pub async fn execute_query(&self, query: &str, params: &[&(dyn tokio_postgres::types::ToSql + Sync)]) -> Result<Vec<Row>, AppError> {
        console_log!("NEON_CLIENT: Executing query: {}", query);

        let rows = self.client.query(query, params).await
            .map_err(|e| AppError::DatabaseError(format!("Query execution failed: {}", e)))?;

        console_log!("NEON_CLIENT: Query executed successfully, returned {} rows", rows.len());
        Ok(rows)
    }

    /// Execute SQL query with no parameters
    pub async fn execute_simple_query(&self, query: &str) -> Result<Vec<Row>, AppError> {
        self.execute_query(query, &[]).await
    }

    /// Test database connectivity
    pub async fn test_connection(&self) -> Result<(), AppError> {
        console_log!("NEON_CLIENT: Testing database connection");
        
        match self.execute_simple_query("SELECT 1 as test").await {
            Ok(rows) => {
                if let Some(row) = rows.first() {
                    let test_value: i32 = row.get(0);
                    if test_value == 1 {
                        console_log!("NEON_CLIENT: Database connection test successful");
                        return Ok(());
                    }
                }
                Err(AppError::DatabaseError("Test query returned unexpected result".to_string()))
            }
            Err(e) => {
                console_log!("NEON_CLIENT: Database connection test failed: {:?}", e);
                Err(e)
            }
        }
    }

    /// Execute raw SQL for migrations or admin tasks
    pub async fn execute_raw(&self, sql: &str) -> Result<u64, AppError> {
        console_log!("NEON_CLIENT: Executing raw SQL: {}", sql);
        
        let affected_rows = self.client.execute(sql, &[]).await
            .map_err(|e| AppError::DatabaseError(format!("Raw SQL execution failed: {}", e)))?;
            
        console_log!("NEON_CLIENT: Raw SQL executed, affected {} rows", affected_rows);
        Ok(affected_rows)
    }

}