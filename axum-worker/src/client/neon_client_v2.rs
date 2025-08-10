use worker::{console_log, Socket};
use crate::error::AppError;
use tokio_postgres::{Client, Config, Row, NoTls};
use std::sync::Arc;

/// Alternative NeonClient implementation with improved buffer handling
/// Based on rusty-worker reference patterns
#[derive(Clone)]
pub struct NeonClientV2 {
    pub(crate) client: Arc<Client>,
}

impl NeonClientV2 {
    /// Create NeonClient with improved connection handling
    pub async fn new(database_url: String) -> Result<Self, AppError> {
        console_log!("NEON_CLIENT_V2: Creating new client with URL: {}", 
            format!("{}...", database_url.chars().take(30).collect::<String>()));
        
        // Parse connection string
        let mut config = database_url.parse::<Config>()
            .map_err(|e| AppError::DatabaseError(format!("Failed to parse database URL: {}", e)))?;
        
        // Try without SSL first to avoid stream issues
        console_log!("NEON_CLIENT_V2: Attempting connection without SSL requirement");
        config.ssl_mode(tokio_postgres::config::SslMode::Prefer);
        
        let host = config.get_hosts().first()
            .ok_or_else(|| AppError::DatabaseError("No host found in database URL".to_string()))?;

        let host_str = match host {
            tokio_postgres::config::Host::Tcp(hostname) => hostname.clone(),
        };

        let port = config.get_ports().first()
            .copied()
            .unwrap_or(5432);

        console_log!("NEON_CLIENT_V2: Connecting to {}:{}", host_str, port);

        // Use socket approach directly (WASM doesn't support direct connection)
        Self::try_socket_connection(&config, &host_str, port).await
    }

    /// Fallback socket connection approach
    async fn try_socket_connection(config: &Config, host_str: &str, port: u16) -> Result<Self, AppError> {
        console_log!("NEON_CLIENT_V2: Attempting socket connection");
        
        let socket = Socket::builder()
            .secure_transport(worker::SecureTransport::StartTls)
            .connect(host_str, port)
            .map_err(|e| AppError::DatabaseError(format!("Socket creation failed: {:?}", e)))?;

        let (client, connection) = config.connect_raw(socket, NoTls).await
            .map_err(|e| AppError::DatabaseError(format!("Socket connection failed: {}", e)))?;

        wasm_bindgen_futures::spawn_local(async move {
            if let Err(e) = connection.await {
                console_log!("NEON_CLIENT_V2: Socket connection error: {}", e);
            }
        });

        Ok(Self {
            client: Arc::new(client),
        })
    }

    /// Execute SQL query with parameters using tokio-postgres
    pub async fn execute_query(&self, query: &str, params: &[&(dyn tokio_postgres::types::ToSql + Sync)]) -> Result<Vec<Row>, AppError> {
        console_log!("NEON_CLIENT_V2: Executing query: {}", query);

        let rows = self.client.query(query, params).await
            .map_err(|e| AppError::DatabaseError(format!("Query execution failed: {}", e)))?;

        console_log!("NEON_CLIENT_V2: Query executed successfully, returned {} rows", rows.len());
        Ok(rows)
    }

    /// Execute SQL query with no parameters
    pub async fn execute_simple_query(&self, query: &str) -> Result<Vec<Row>, AppError> {
        self.execute_query(query, &[]).await
    }

    /// Test database connectivity
    pub async fn test_connection(&self) -> Result<(), AppError> {
        console_log!("NEON_CLIENT_V2: Testing database connection");
        
        match self.execute_simple_query("SELECT 1 as test").await {
            Ok(rows) => {
                if let Some(row) = rows.first() {
                    let test_value: i32 = row.get(0);
                    if test_value == 1 {
                        console_log!("NEON_CLIENT_V2: Database connection test successful");
                        return Ok(());
                    }
                }
                Err(AppError::DatabaseError("Test query returned unexpected result".to_string()))
            }
            Err(e) => {
                console_log!("NEON_CLIENT_V2: Database connection test failed: {:?}", e);
                Err(e)
            }
        }
    }
}