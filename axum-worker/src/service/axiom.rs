use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use worker::*;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64_STANDARD};
use sha2::Sha256;
use pbkdf2::pbkdf2;
use hmac::Hmac;

#[derive(Debug, Clone)]
pub struct AxiomService {
    base_url: String,
    session: Option<AxiomSession>,
    // Only one password algorithm is now supported (SHA256 -> Base64 per Python reference)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AxiomSession {
    pub access_token: String,
    pub refresh_token: String,
    pub user_id: String,
    pub expires_at: i64,
}

// Step 1 Login Request (email + b64Password)
#[derive(Debug, Serialize)]
struct LoginStep1Request {
    email: String,
    #[serde(rename = "b64Password")]
    b64_password: String, // Base64 encoded SHA256 hash
}

// Step 1 Login Response (as observed in Python client)
#[derive(Debug, Deserialize)]
pub struct LoginStep1ResponseBody {
    #[serde(rename = "otpJwtToken")]
    pub otp_jwt_token: Option<String>,
    #[serde(flatten)]
    pub rest: serde_json::Value,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoginStep1Response {
    pub success: bool,
    pub jwt: Option<String>, // otpJwtToken or cookie token
    pub message: Option<String>,
}

// Step 2 Login Request (OTP verification)
#[derive(Debug, Serialize)]
struct LoginStep2Request {
    #[serde(rename = "code")]
    otp: String,
    email: String,
    #[serde(rename = "b64Password")]
    b64_password: String,
}

// Step 2 Login Response
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoginStep2ResponseBody {
    #[serde(rename = "accessToken")]
    pub access_token: Option<String>,
    #[serde(rename = "refreshToken")]
    pub refresh_token: Option<String>,
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
    #[serde(flatten)]
    pub rest: serde_json::Value,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoginStep2Response {
    pub success: bool,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub user_id: Option<String>,
    pub message: Option<String>,
}

// Token refresh request
#[derive(Debug, Serialize)]
struct RefreshTokenRequest {
    refresh_token: String,
}

// Token refresh response
#[derive(Debug, Deserialize)]
struct RefreshTokenResponse {
    success: bool,
    access_token: Option<String>,
    message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AxiomUser {
    pub id: String,
    pub email: String,
    pub name: String,
    pub account_balance: Option<f64>,
    pub trading_enabled: bool,
}

impl AxiomService {
    /// Create a new Axiom service instance
    pub fn new() -> Self {
    Self { base_url: "https://api6.axiom.trade".to_string(), session: None }
    }

    /// Encode password for transmission.
    /// According to the upstream Python reference (AuthManager.get_b64_password),
    /// the password should be SHA256 hashed (ISO-8859-1 bytes, equivalent to raw bytes for ASCII)
    /// and then Base64 encoded (padding retained). This produces a 44-char string for typical inputs.
    /// We log length to help verify. If future requirements need raw Base64(password), a secondary
    /// function can be introduced and toggled via configuration.
    pub fn hash_password(password: &str) -> String {
        // PBKDF2-HMAC-SHA256 with fixed salt & iterations per new reference
        // salt bytes from Python snippet
        const SALT: [u8; 32] = [217, 3, 161, 123, 53, 200, 206, 36, 143, 2, 220, 252, 240, 109, 204, 23, 217, 174, 79, 158, 18, 76, 149, 117, 73, 40, 207, 77, 34, 194, 196, 163];
        const ITERATIONS: u32 = 600_000;
        worker::console_log!("🔐 AXIOM DEBUG: raw password='{}' (PBKDF2)", password);

        // ISO-8859-1 strict encoding
        let mut latin1: Vec<u8> = Vec::with_capacity(password.len());
        for ch in password.chars() {
            let cp = ch as u32;
            if cp <= 0xFF { latin1.push(cp as u8); } else {
                worker::console_log!("🔐 AXIOM ERROR: password contains non ISO-8859-1 character U+{:04X}; abort PBKDF2", cp);
                return String::new();
            }
        }

    let mut derived = [0u8; 32];
        pbkdf2::<Hmac<Sha256>>(&latin1, &SALT, ITERATIONS, &mut derived)
            .expect("PBKDF2 derivation failed");
        let b64 = BASE64_STANDARD.encode(derived);
        worker::console_log!("🔐 AXIOM: PBKDF2-HMAC-SHA256 derived key -> base64='{}' len={} iterations={} salt_len={}", b64, b64.len(), ITERATIONS, SALT.len());
        b64
    }

    // All alternative encodings removed; only canonical hashing remains.

    /// Create common headers for Axiom API requests
    fn create_headers(&self) -> Result<Headers> {
        let headers = Headers::new();
        headers.set("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0 Safari/537.36")?;
        headers.set("Accept", "application/json, text/plain, */*")?;
        headers.set("Accept-Language", "en-US,en;q=0.9")?;
        headers.set("Accept-Encoding", "gzip, deflate, br, zstd")?;
        headers.set("Origin", "https://axiom.trade")?;
        headers.set("Referer", "https://axiom.trade/")?;
        headers.set("Content-Type", "application/json")?;
        headers.set("Sec-Ch-Ua", "\"Chromium\";v=124, \"Not-A.Brand\";v=99")?;
        headers.set("Sec-Ch-Ua-Platform", "\"Windows\"")?;
        headers.set("Sec-Ch-Ua-Mobile", "?0")?;
        headers.set("X-Requested-With", "XMLHttpRequest")?; // mimic typical AJAX
        Ok(headers)
    }

    /// Create headers with authentication token
    fn create_auth_headers(&self) -> Result<Headers> {
    let headers = self.create_headers()?;
        if let Some(session) = &self.session {
            headers.set(
                "Cookie",
                &format!("auth-access-token={}", session.access_token),
            )?;
        }
        Ok(headers)
    }

    /// Step 1: Login with email and password to get OTP JWT token (or cookie) for OTP verification
    pub async fn login_step1(&mut self, email: &str, password: &str) -> Result<LoginStep1Response> {
        // Endpoints observed in Python client: /login-password-v2 on api6, others as fallback
        let mut candidates = vec![("https://api6.axiom.trade", "/login-password-v2")];
        // Randomize order to distribute load and reduce rate limit triggers on a single host
        #[allow(unused)]
        {
            use rand::seq::SliceRandom;
            let mut rng = rand::thread_rng();
            candidates.shuffle(&mut rng);
        }

    // IMPORTANT: Do not force lowercase on the email. The upstream Python reference
    // sends the email as provided by the user (only trimming). Lowercasing could
    // cause a credential mismatch if the backend performs a case-sensitive comparison
    // or uses the original casing as part of the password hashing / lookup pipeline.
    let clean_email = email.trim().to_string();
    let encoded_pw = Self::hash_password(password);
    let mut last_err: Option<String> = None;
        let headers_base = self.create_headers()?;
        headers_base.set("Cookie", "auth-otp-login-token=").ok();

    worker::console_log!("🔐 AXIOM DEBUG: starting step1 for email='{}' raw_password='{}'", clean_email, password);

    // Single attempt per endpoint using canonical encoding
    for (base, path) in candidates.iter() {
                let url = format!("{}{}", base, path);
                console_log!("🔐 AXIOM: Trying step1 endpoint: {}", url);

                let headers = headers_base.clone();
        let display_fragment = if encoded_pw.len() > 10 { format!("{}...{}", &encoded_pw[..6], &encoded_pw[encoded_pw.len()-4..]) } else { encoded_pw.clone() };
        worker::console_log!("🔐 AXIOM: step1 prepared password full='{}' fragment={} len={}", encoded_pw, display_fragment, encoded_pw.len());
        let body = LoginStep1Request { email: clean_email.clone(), b64_password: encoded_pw.clone() };

                let mut request_init = RequestInit::new();
                request_init
                    .with_method(Method::Post)
                    .with_headers(headers.clone())
                    .with_body(Some(JsValue::from_str(&serde_json::to_string(&body).unwrap())));

                match Request::new_with_init(&url, &request_init) {
                    Ok(req) => {
                        match Fetch::Request(req).send().await {
                            Ok(mut response) => {
                                let status = response.status_code();
                                let text = response.text().await.unwrap_or_default();

                                if status == 200 {
                                    // Try parse JSON for otpJwtToken
                                    let parsed: serde_json::Result<LoginStep1ResponseBody> = serde_json::from_str(&text);
                                    let mut jwt: Option<String> = parsed.ok().and_then(|b| b.otp_jwt_token);

                                    // Also try Set-Cookie for auth-otp-login-token
                                    if jwt.is_none() {
                                        if let Ok(Some(cookie_header)) = response.headers().get("set-cookie") {
                                            if let Some(tok) = extract_cookie(&cookie_header, "auth-otp-login-token") {
                                                jwt = Some(tok);
                                            }
                                        }
                                    }

                                    if let Some(t) = jwt.clone() {
                                        self.base_url = base.to_string();
                                        console_log!("🔐 AXIOM: step1 succeeded via {}", url);
                                        return Ok(LoginStep1Response { success: true, jwt: Some(t), message: None });
                                    } else {
                                        last_err = Some(format!("{} -> 200 but no otp token in body or cookies: {}", url, text));
                                    }
                                } else {
                                    let lowered = text.to_lowercase();
                                    // Treat these as rate-limit style responses
                                    let rate_limited = (status == 429 || status == 500)
                                        || lowered.contains("rate limit")
                                        || lowered.contains("too many requests")
                                        || lowered.contains("too many attempts")
                                        || lowered.contains("try again later");
                                    if rate_limited {
                                        console_log!("🔐 AXIOM: Rate limited on {} (status {}): {}", url, status, text);
                                        return Ok(LoginStep1Response { success: false, jwt: None, message: Some("Rate limited by Axiom API. Please wait ~60 seconds before retrying.".to_string()) });
                                    }
                                    last_err = Some(format!("{} -> {} {}", url, status, text));
                                }
                            }
                            Err(e) => {
                                last_err = Some(format!("{} -> transport error: {}", url, e));
                            }
                        }
                    }
                    Err(e) => {
                        last_err = Some(format!("{} -> request build error: {}", url, e));
                    }
                }
    }

        Err(Error::RustError(format!(
            "Login step1 failed across endpoints: {}",
            last_err.unwrap_or_else(|| "no details".to_string())
        )))
    }

    /// Step 2: Accepts base64-encoded password (same value used in step1) and submits OTP
    pub async fn login_step2(
        &mut self,
        jwt: &str,
        otp_code: &str,
        email: &str,
        base64_password: &str,
    ) -> Result<LoginStep2Response> {
        let candidates = vec![
            ("https://api10.axiom.trade", "/login-otp"),
            ("https://api6.axiom.trade", "/login-otp"),
            ("https://api.axiom.trade", "/login-otp"),
        ];
        // Choose a single random endpoint to avoid noisy failures and distribute load
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        let (base, path) = candidates
            .choose(&mut rng)
            .copied()
            .unwrap_or(("https://api6.axiom.trade", "/login-otp"));

        let clean_email = email.trim().to_string();
        let body = LoginStep2Request {
            otp: otp_code.to_string(),
            email: clean_email.clone(),
            b64_password: base64_password.to_string(),
        };
        let headers = self.create_headers()?;
        headers.set("Cookie", &format!("auth-otp-login-token={}", jwt))?;

        let url = format!("{}{}", base, path);
        console_log!("🔐 AXIOM: Step2 using endpoint: {}", url);

        let mut request_init = RequestInit::new();
        request_init
            .with_method(Method::Post)
            .with_headers(headers)
            .with_body(Some(JsValue::from_str(
                &serde_json::to_string(&body).unwrap(),
            )));

        match Request::new_with_init(&url, &request_init) {
            Ok(req) => match Fetch::Request(req).send().await {
                Ok(mut response) => {
                    let status = response.status_code();
                    let text = response.text().await.unwrap_or_default();
                    if status == 200 {
                        let parsed: serde_json::Result<LoginStep2ResponseBody> = serde_json::from_str(&text);
                        let mut access_token = parsed.as_ref().ok().and_then(|b| b.access_token.clone());
                        let mut refresh_token = parsed.as_ref().ok().and_then(|b| b.refresh_token.clone());
                        let mut user_id = parsed.as_ref().ok().and_then(|b| b.user_id.clone());
                        if let Ok(Some(cookie_header)) = response.headers().get("set-cookie") {
                            if access_token.is_none() { access_token = extract_cookie(&cookie_header, "auth-access-token"); }
                            if refresh_token.is_none() { refresh_token = extract_cookie(&cookie_header, "auth-refresh-token"); }
                        }
                        if user_id.is_none() { user_id = Some(email.to_string()); }
                        let success = access_token.is_some() && refresh_token.is_some();
                        if success {
                            self.session = Some(AxiomSession {
                                access_token: access_token.clone().unwrap(),
                                refresh_token: refresh_token.clone().unwrap(),
                                user_id: user_id.clone().unwrap_or_else(|| email.to_string()),
                                expires_at: chrono::Utc::now().timestamp() + 3600,
                            });
                            self.base_url = base.to_string();
                            console_log!("🔐 AXIOM: step2 succeeded via {}", url);
                            Ok(LoginStep2Response { success: true, access_token, refresh_token, user_id, message: None })
                        } else {
                            Err(Error::RustError("Login step2 succeeded but tokens not found".to_string()))
                        }
                    } else {
                        let lowered = text.to_lowercase();
                        let rate_limited = (status == 429 || status == 500)
                            || lowered.contains("rate limit")
                            || lowered.contains("too many requests")
                            || lowered.contains("too many attempts")
                            || lowered.contains("try again later");
                        if rate_limited {
                            console_log!("🔐 AXIOM: Step2 rate limited on {} (status {}): {}", url, status, text);
                            Ok(LoginStep2Response { success: false, access_token: None, refresh_token: None, user_id: None, message: Some("Try again later".to_string()) })
                        } else {
                            Err(Error::RustError(format!("Login step2 failed ({}): {}", status, text)))
                        }
                    }
                }
                Err(e) => Err(Error::RustError(format!("Step2 transport error: {}", e))),
            },
            Err(e) => Err(Error::RustError(format!("Step2 request build error: {}", e))),
        }
    }



    /// Refresh the access token using refresh token
    pub async fn refresh_token(&mut self) -> Result<()> {
        let session = self
            .session
            .as_ref()
            .ok_or_else(|| Error::RustError("No active session".to_string()))?;

        let url = format!("{}/api/auth/refresh", self.base_url);

        let body = RefreshTokenRequest {
            refresh_token: session.refresh_token.clone(),
        };

        let headers = self.create_headers()?;

        let mut request_init = RequestInit::new();
        request_init
            .with_method(Method::Post)
            .with_headers(headers)
            .with_body(Some(JsValue::from_str(
                &serde_json::to_string(&body).unwrap(),
            )));

        let request = Request::new_with_init(&url, &request_init)?;
        let mut response = Fetch::Request(request).send().await?;

        if response.status_code() == 200 {
            let text = response.text().await?;
            let refresh_response: RefreshTokenResponse =
                serde_json::from_str(&text).map_err(|e| {
                    Error::RustError(format!("Failed to parse refresh response: {}", e))
                })?;

            if refresh_response.success {
                if let Some(new_access_token) = refresh_response.access_token {
                    if let Some(session) = &mut self.session {
                        session.access_token = new_access_token;
                        session.expires_at = chrono::Utc::now().timestamp() + 3600;
                    }
                }
            }
            Ok(())
        } else {
            Err(Error::RustError(format!(
                "Token refresh failed with status: {}",
                response.status_code()
            )))
        }
    }

    /// Get portfolio information
    pub async fn get_portfolio(&self) -> Result<serde_json::Value> {
        let url = format!("{}/api/portfolio", self.base_url);

        let headers = self.create_auth_headers()?;

        let mut request_init = RequestInit::new();
        request_init.with_method(Method::Get).with_headers(headers);

        let request = Request::new_with_init(&url, &request_init)?;
        let mut response = Fetch::Request(request).send().await?;

        if response.status_code() == 200 {
            let text = response.text().await?;
            let portfolio: serde_json::Value = serde_json::from_str(&text)
                .map_err(|e| Error::RustError(format!("Failed to parse portfolio: {}", e)))?;
            Ok(portfolio)
        } else {
            Err(Error::RustError(format!(
                "Failed to get portfolio: {}",
                response.status_code()
            )))
        }
    }

    /// Check if the current session is valid
    pub fn is_authenticated(&self) -> bool {
        if let Some(session) = &self.session {
            let now = chrono::Utc::now().timestamp();
            now < session.expires_at
        } else {
            false
        }
    }

    /// Get the current session if authenticated
    pub fn get_session(&self) -> Option<&AxiomSession> {
        self.session.as_ref()
    }

    /// Set tokens manually
    pub fn set_tokens(&mut self, access_token: String, refresh_token: String) {
        self.session = Some(AxiomSession {
            access_token,
            refresh_token,
            user_id: String::new(), // Will be set later
            expires_at: chrono::Utc::now().timestamp() + 3600,
        });
    }

    /// Get current tokens
    pub fn get_tokens(&self) -> Option<(String, String)> {
        self.session
            .as_ref()
            .map(|s| (s.access_token.clone(), s.refresh_token.clone()))
    }
}

/// Complete authentication flow helper
pub async fn authenticate_with_axiom(
    email: &str,
    password: &str,
    gmail_service: &super::gmail::GmailService,
) -> Result<AxiomSession> {
    let mut axiom = AxiomService::new();

    // Step 1: Initial login to trigger OTP
    console_log!("🔐 AXIOM: Starting login step 1...");
    let step1_response = axiom.login_step1(email, password).await?;

    if !step1_response.success {
        return Err(Error::RustError(
            step1_response
                .message
                .unwrap_or_else(|| "Login step 1 failed".to_string()),
        ));
    }

    let jwt = step1_response
        .jwt
        .ok_or_else(|| Error::RustError("No JWT token in step 1 response".to_string()))?;

    console_log!("🔐 AXIOM: Login step 1 successful, waiting for OTP email...");

    // Wait for OTP email and extract code
    let mut otp_code = None;
    for attempt in 0..10 {
        // Try for up to 20 seconds
        if attempt > 0 {
            // Wait 2 seconds between attempts
            console_log!("🔐 AXIOM: Attempt {} to fetch OTP code...", attempt + 1);
        }

        if let Ok(Some(code)) = gmail_service.get_axiom_2fa_code(email).await {
            otp_code = Some(code);
            break;
        }

        // In a real implementation, we'd use a proper delay here
    }

    let otp = otp_code
        .ok_or_else(|| Error::RustError("Failed to retrieve OTP code from email".to_string()))?;

    console_log!("🔐 AXIOM: OTP code retrieved: {}", otp);

    // Step 2: Submit OTP to complete authentication
    let step2_response = axiom.login_step2(&jwt, &otp, email, password).await?;

    if !step2_response.success {
        return Err(Error::RustError(
            step2_response
                .message
                .unwrap_or_else(|| "Login step 2 failed".to_string()),
        ));
    }

    axiom.session.ok_or_else(|| {
        Error::RustError("Authentication completed but no session created".to_string())
    })
}

// Utility: Extract a cookie value from a Set-Cookie header that may contain multiple cookies
fn extract_cookie(set_cookie_header: &str, name: &str) -> Option<String> {
    // The header may contain multiple Set-Cookie entries merged; split on commas but beware commas in attributes.
    // A simpler approach: look for "name=" and take until ';'
    if let Some(start) = set_cookie_header.find(&format!("{}=", name)) {
        let rest = &set_cookie_header[start + name.len() + 1..];
        let end = rest.find(';').unwrap_or(rest.len());
        let value = &rest[..end];
        return Some(value.to_string());
    }
    None
}
