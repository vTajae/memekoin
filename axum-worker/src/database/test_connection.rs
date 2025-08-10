//! Database Connection Diagnostic Tool
//! 
//! This module provides diagnostic functions to test database connectivity
//! and identify connection issues in the Cloudflare Workers environment.

use worker::{console_log, Env};
use crate::{error::AppError, client::{neon_client::NeonClient, neon_client_v2::NeonClientV2}};

/// Test database connection with detailed logging
pub async fn test_database_connection_detailed(env: &Env) -> Result<(), AppError> {
    console_log!("=== DATABASE CONNECTION DIAGNOSTIC ===");
    
    // Step 1: Test environment variable access
    console_log!("Step 1: Testing environment variable access...");
    
    let database_url = match env.secret("DEV_DATABASE_URL") {
        Ok(secret) => {
            console_log!("✅ DEV_DATABASE_URL found as secret");
            secret.to_string()
        }
        Err(_) => {
            console_log!("⚠️  DEV_DATABASE_URL not found as secret, trying as variable...");
            match env.var("DEV_DATABASE_URL") {
                Ok(var) => {
                    console_log!("✅ DEV_DATABASE_URL found as variable");
                    var.to_string()
                }
                Err(e) => {
                    console_log!("❌ DEV_DATABASE_URL not found: {:?}", e);
                    return Err(AppError::DatabaseError("DEV_DATABASE_URL environment variable not found".to_string()));
                }
            }
        }
    };
    
    console_log!("Database URL format: {}...", 
        database_url.chars().take(30).collect::<String>());
    
    // Step 2: Parse connection string
    console_log!("Step 2: Parsing connection string...");
    let parsed_url = url::Url::parse(&database_url)
        .map_err(|e| {
            console_log!("❌ Failed to parse database URL: {:?}", e);
            AppError::DatabaseError(format!("Invalid database URL format: {}", e))
        })?;
    
    console_log!("✅ Connection string parsed successfully");
    console_log!("Host: {}", parsed_url.host_str().unwrap_or("unknown"));
    console_log!("Port: {}", parsed_url.port().unwrap_or(5432));
    console_log!("Database: {}", parsed_url.path().trim_start_matches('/'));
    
    // Step 3: Test NeonClientV2 creation (improved version)
    console_log!("Step 3: Testing NeonClientV2 creation (improved connection handling)...");
    let client_v2 = NeonClientV2::new(database_url.clone()).await
        .map_err(|e| {
            console_log!("❌ NeonClientV2 creation failed: {:?}", e);
            // Don't return error, try original client
            e
        });
    
    match client_v2 {
        Ok(client) => {
            console_log!("✅ NeonClientV2 created successfully");
            
            // Step 4: Test actual connection
            console_log!("Step 4: Testing database connectivity with V2...");
            match client.test_connection().await {
                Ok(_) => {
                    console_log!("✅ Database connection successful with NeonClientV2!");
                    console_log!("=== CONNECTION DIAGNOSTIC COMPLETE (V2 SUCCESS) ===");
                    return Ok(());
                }
                Err(e) => {
                    console_log!("❌ V2 connection test failed: {:?}, trying original client", e);
                }
            }
        }
        Err(e) => {
            console_log!("❌ NeonClientV2 failed, trying original NeonClient: {:?}", e);
        }
    }
    
    // Fallback to original client
    console_log!("Step 5: Fallback to original NeonClient...");
    let client = NeonClient::new(database_url.clone()).await
        .map_err(|e| {
            console_log!("❌ Original NeonClient creation also failed: {:?}", e);
            e
        })?;
    
    console_log!("✅ Original NeonClient created successfully");
    
    // Step 6: Test actual connection
    console_log!("Step 6: Testing database connectivity with original client...");
    client.test_connection().await
        .map_err(|e| {
            console_log!("❌ Database connection test failed: {:?}", e);
            e
        })?;
    
    console_log!("✅ Database connection successful with original client!");
    console_log!("=== CONNECTION DIAGNOSTIC COMPLETE ===");
    
    Ok(())
}

/// Quick connection string validation
pub fn validate_connection_string(connection_string: &str) -> Result<DatabaseInfo, AppError> {
    let url = url::Url::parse(connection_string)
        .map_err(|e| AppError::DatabaseError(format!("Invalid URL format: {}", e)))?;
    
    let host = url.host_str()
        .ok_or_else(|| AppError::DatabaseError("No host in connection string".to_string()))?;
    
    let port = url.port().unwrap_or(5432);
    let database = url.path().trim_start_matches('/');
    let username = url.username();
    
    // Check for Neon-specific patterns
    let is_neon = host.contains("neon.tech") || host.contains("pooler");
    let has_ssl = connection_string.contains("sslmode=require");
    
    Ok(DatabaseInfo {
        host: host.to_string(),
        port,
        database: database.to_string(),
        username: username.to_string(),
        is_neon,
        has_ssl,
        is_valid: !host.is_empty() && !username.is_empty(),
    })
}

#[derive(Debug)]
pub struct DatabaseInfo {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub is_neon: bool,
    pub has_ssl: bool,
    pub is_valid: bool,
}

impl DatabaseInfo {
    pub fn print_summary(&self) {
        console_log!("=== DATABASE INFO SUMMARY ===");
        console_log!("Host: {}", self.host);
        console_log!("Port: {}", self.port);
        console_log!("Database: {}", self.database);
        console_log!("Username: {}", self.username);
        console_log!("Is Neon DB: {}", self.is_neon);
        console_log!("SSL Enabled: {}", self.has_ssl);
        console_log!("Valid Format: {}", self.is_valid);
        console_log!("=== END DATABASE INFO ===");
    }
}