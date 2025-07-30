use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Clone)]
pub struct AuthenticationService {
    #[allow(dead_code)]
    secret: String,
}

impl AuthenticationService {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }

    pub fn generate_token_for(&self, username: String) -> Result<String, String> {
        // Use js_sys::Date for WASM compatibility
        let now = js_sys::Date::now() / 1000.0; // Convert to seconds
        let expiration = (now + 86400.0) as usize; // 24 hours

        let claims = Claims {
            sub: username,
            exp: expiration,
        };

        // For now, create a simple token
        // TODO: Implement proper JWT signing
        Ok(format!("token_for_{}_exp_{}", claims.sub, claims.exp))
    }
}