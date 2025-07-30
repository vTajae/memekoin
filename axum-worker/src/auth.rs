use serde::{Deserialize, Serialize};
use base64::{Engine as _, engine::general_purpose};
use crate::models::User;

// JWT Claims structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (user ID)
    pub username: String,
    pub exp: usize, // Expiration time
    pub iat: usize, // Issued at
}

// Simplified password hashing for demo/testing purposes
// TODO: Implement proper Web Crypto API hashing once basic functionality works
pub async fn hash_password(password: &str) -> std::result::Result<String, String> {
    // Simple base64 encoding with salt for demo - NOT production ready
    let salt = "demo_salt_12345";
    let combined = format!("{}:{}", salt, password);
    Ok(general_purpose::STANDARD.encode(combined.as_bytes()))
}

pub async fn verify_password(password: &str, stored_hash: &str) -> std::result::Result<bool, String> {
    // Simple verification matching the simple hashing approach
    let expected_hash = hash_password(password).await?;
    Ok(stored_hash == expected_hash)
}

// Simplified JWT token functions for demo/testing purposes
// TODO: Implement proper HMAC-SHA256 signing once basic functionality works
pub async fn create_jwt_token(user: &User, jwt_secret: &str) -> std::result::Result<String, String> {
    let now = js_sys::Date::now() / 1000.0;
    let exp = (now + 86400.0) as usize; // 24 hours
    let iat = now as usize;

    let claims = Claims {
        sub: user.id.clone(),
        username: user.username.clone(),
        exp,
        iat,
    };

    // Create header and payload
    let header = serde_json::json!({"alg": "HS256", "typ": "JWT"});
    let header_encoded = general_purpose::URL_SAFE_NO_PAD.encode(serde_json::to_string(&header).unwrap());
    let payload_encoded = general_purpose::URL_SAFE_NO_PAD.encode(serde_json::to_string(&claims).unwrap());
    
    let message = format!("{}.{}", header_encoded, payload_encoded);
    
    // Simple signature for demo - NOT production ready
    let signature = format!("{}:{}", jwt_secret, message);
    let signature_encoded = general_purpose::URL_SAFE_NO_PAD.encode(signature.as_bytes());
    
    Ok(format!("{}.{}", message, signature_encoded))
}

pub async fn verify_jwt_token(token: &str, jwt_secret: &str) -> std::result::Result<Claims, String> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err("Invalid token format".to_string());
    }

    let message = format!("{}.{}", parts[0], parts[1]);
    let signature_bytes = general_purpose::URL_SAFE_NO_PAD.decode(parts[2])
        .map_err(|e| format!("Invalid signature encoding: {}", e))?;
    
    let signature = String::from_utf8(signature_bytes)
        .map_err(|e| format!("Invalid signature format: {}", e))?;

    // Verify signature (simple approach)
    let expected_signature = format!("{}:{}", jwt_secret, message);
    if signature != expected_signature {
        return Err("Invalid signature".to_string());
    }

    // Decode claims
    let payload = general_purpose::URL_SAFE_NO_PAD.decode(parts[1])
        .map_err(|e| format!("Invalid payload encoding: {}", e))?;
    let claims: Claims = serde_json::from_slice(&payload)
        .map_err(|e| format!("Invalid claims format: {}", e))?;

    // Check expiration
    let now = (js_sys::Date::now() / 1000.0) as usize;
    if claims.exp < now {
        return Err("Token expired".to_string());
    }

    Ok(claims)
}
