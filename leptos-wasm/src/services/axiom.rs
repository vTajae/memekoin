use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{info, error};
use sha2::{Sha256, Digest};
use base64::Engine;
use crate::services::base::ApiClient;

/// Axiom Trade authentication service (frontend implementation)
#[derive(Clone)]
pub struct AxiomService {
    client: Client,
    api_client: ApiClient, // For Gmail integration via backend
    base_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginStep1Request {
    pub email: String,
    pub password: String, // This will be hashed
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginStep1Response {
    pub success: bool,
    pub message: Option<String>,
    pub jwt: Option<String>,
    pub user_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginStep2Request {
    pub jwt: String,
    pub otp: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginStep2Response {
    pub success: bool,
    pub message: Option<String>,
    pub access_token: Option<String>,
    pub user_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AxiomAuthRequest {
    pub step: String,
    pub email: Option<String>,
    pub password: Option<String>,
    pub session_id: Option<String>,
    pub otp_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AxiomAuthResponse {
    pub success: bool,
    pub next_step: Option<String>,
    pub session_id: Option<String>,
    pub axiom_jwt: Option<String>,
    pub message: Option<String>,
    pub user_data: Option<AxiomUserData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AxiomUserData {
    pub email: String,
    pub name: String,
    pub account_id: String,
    pub trading_enabled: bool,
}

impl AxiomService {
    pub fn new() -> Self {
        let client = Client::builder()
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            api_client: ApiClient::new(),
            // Backend base_url (kept for potential Gmail helper endpoints)
            base_url: "http://127.0.0.1:8787".to_string(),
        }
    }

    /// Hash password using SHA256 and encode as base64 (kept for reference). The backend will
    /// perform the canonical derivation (PBKDF2-HMAC-SHA256) and send it to Axiom; the frontend
    /// just forwards the raw password in step2.
    fn hash_password(password: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        let result = hasher.finalize();
        base64::engine::general_purpose::STANDARD.encode(result)
    }

    /// Get 2FA code from Gmail via backend service
    #[tracing::instrument(name = "axiom.get_2fa_code_from_gmail", skip(self))]
    pub async fn get_2fa_code_from_gmail(&self, user_email: &str) -> Result<String, String> {
        info!("📧 AXIOM: Attempting to retrieve 2FA code from Gmail for: {}", user_email);
        
        #[derive(Serialize)]
        struct GmailRequest {
            user_email: String,
        }
        
        #[derive(Deserialize)]
        struct GmailResponse {
            success: bool,
            message: Option<String>,
            otp_code: Option<String>,
        }
        
        let request = GmailRequest {
            user_email: user_email.to_string(),
        };
        
        // Call backend Gmail service to get 2FA code
        match self.api_client.post::<GmailRequest, GmailResponse>("/auth/gmail/axiom-2fa", &request).await {
            Ok(response) => {
                if response.success {
                    if let Some(otp_code) = response.otp_code {
                        info!("📧 AXIOM: Successfully retrieved 2FA code from Gmail");
                        Ok(otp_code)
                    } else {
                        error!("📧 AXIOM: Backend returned success but no OTP code");
                        Err("No 2FA code found in emails".to_string())
                    }
                } else {
                    let error_msg = response.message.unwrap_or_else(|| "Failed to retrieve 2FA code".to_string());
                    error!("📧 AXIOM: Backend Gmail service failed: {}", error_msg);
                    Err(error_msg)
                }
            }
            Err(e) => {
                error!("📧 AXIOM: Gmail service request failed: {}", e);
                Err(format!("Failed to contact Gmail service: {}", e))
            }
        }
    }


    /// Step 1: Login with email and password to get JWT for OTP (via backend)
    #[tracing::instrument(name = "axiom.login_step1", skip(self, password))]
    pub async fn login_step1(&self, email: &str, password: &str) -> Result<LoginStep1Response, String> {
        let _hashed_password = Self::hash_password(password);
        let req = AxiomAuthRequest {
            step: "email".to_string(),
            email: Some(email.to_string()),
            password: Some(password.to_string()),
            session_id: None,
            otp_code: None,
        };
        // Call backend
        let resp: AxiomAuthResponse = self.api_client.post("/auth/axiom", &req).await?;
        if resp.success {
            // Require backend-provided axiom_jwt (JWT for step2). No fallback.
            match resp.axiom_jwt {
                Some(jwt) => Ok(LoginStep1Response {
                    success: true,
                    message: resp.message,
                    jwt: Some(jwt),
                    user_id: resp.user_data.map(|u| u.account_id),
                }),
                None => Err("Missing axiom_jwt from backend".to_string()),
            }
        } else {
            Err(resp.message.unwrap_or_else(|| "Login step 1 failed".to_string()))
        }
    }


    /// Step 2: Submit OTP code with JWT to complete authentication (via backend)
    /// The JWT from step1 is carried as session_id here; the backend will attach it as
    /// the auth-otp-login-token cookie when calling Axiom /login-otp.
    /// We send the raw password here; backend derives the identical base64Password used in step 1.
    #[tracing::instrument(name = "axiom.login_step2", skip(self, otp_code))]
    pub async fn login_step2(&self, jwt: &str, otp_code: &str, email: &str, password: &str) -> Result<LoginStep2Response, String> {
        let req = AxiomAuthRequest {
            step: "2fa".to_string(),
            email: Some(email.to_string()),
            password: Some(password.to_string()),
            session_id: Some(jwt.to_string()),
            otp_code: Some(otp_code.to_string()),
        };
        // Send JWT in header so backend can prefer it
        let headers = [("x-axiom-otp-jwt", jwt)];
        let resp: AxiomAuthResponse = self.api_client.post_with_headers("/auth/axiom", &req, &headers).await?;
        if resp.success {
            Ok(LoginStep2Response {
                success: true,
                message: resp.message,
                access_token: resp.session_id,
                user_id: resp.user_data.map(|u| u.account_id),
            })
        } else {
            Err(resp.message.unwrap_or_else(|| "Login step 2 failed".to_string()))
        }
    }

    /// Handle multi-step authentication (frontend makes live calls; backend kept Send-safe)
    #[tracing::instrument(name = "axiom.handle_auth_step", skip(self, request))]
    pub async fn handle_auth_step(&self, request: AxiomAuthRequest) -> Result<AxiomAuthResponse, String> {
        info!("🔐 AXIOM: Handling auth step on frontend: {}", request.step);
        match request.step.as_str() {
            "email" => {
                let email = request.email.ok_or("Email is required for step1")?;
                let password = request.password.ok_or("Password is required for step1")?;
                let step1 = self.login_step1(&email, &password).await?;
                if step1.success {
                    Ok(AxiomAuthResponse {
                        success: true,
                        next_step: Some("2fa".to_string()),
                        // session_id will carry the step1 JWT for the UI flow
                        session_id: step1.jwt.clone(),
                        axiom_jwt: step1.jwt,
                        message: Some("Please enter the 2FA code sent to your email".to_string()),
                        user_data: None,
                    })
                } else {
                    Ok(AxiomAuthResponse {
                        success: false,
                        next_step: None,
                        session_id: None,
                        axiom_jwt: None,
                        message: step1.message.or(Some("Authentication failed".to_string())),
                        user_data: None,
                    })
                }
            }
            "2fa" => {
                let jwt = request.session_id.ok_or("JWT is required for 2FA")?;
                let otp = request.otp_code.ok_or("OTP code is required for 2FA")?;
                // Don't hard-require email on the client; backend can derive from session if missing
                let email = request.email.clone().unwrap_or_default();
                // Send raw password; backend will derive proper hash
                let password = request.password.unwrap_or_default();
                let step2 = self.login_step2(&jwt, &otp, &email, &password).await?;
                if step2.success {
                    Ok(AxiomAuthResponse {
                        success: true,
                        next_step: Some("complete".to_string()),
                        session_id: step2.access_token,
                        axiom_jwt: None,
                        message: Some("Authentication successful!".to_string()),
                        user_data: Some(AxiomUserData {
                            email: email,
                            name: "Axiom Trader".to_string(),
                            account_id: step2.user_id.unwrap_or_else(|| "AXIOM_LIVE".to_string()),
                            trading_enabled: true,
                        }),
                    })
                } else {
                    Ok(AxiomAuthResponse {
                        success: false,
                        next_step: None,
                        session_id: Some(jwt),
                        axiom_jwt: None,
                        message: step2.message.or(Some("2FA verification failed".to_string())),
                        user_data: None,
                    })
                }
            }
            _ => Err(format!("Unknown authentication step: {}", request.step)),
        }
    }
    
    /// Legacy handle_auth_step implementation (replaced by backend API)
    #[deprecated(note = "Use handle_auth_step which calls backend API instead")]
    async fn handle_auth_step_old(&self, request: AxiomAuthRequest) -> Result<AxiomAuthResponse, String> {
        info!("🔐 AXIOM: Handling auth step: {}", request.step);
        
        match request.step.as_str() {
            "email" => {
                let email = request.email.ok_or("Email is required for email step")?;
                let password = request.password.ok_or("Password is required for email step")?;
                
                match self.login_step1(&email, &password).await {
                    Ok(step1_response) => {
                        if step1_response.success {
                            Ok(AxiomAuthResponse {
                                success: true,
                                next_step: Some("2fa".to_string()),
                                session_id: step1_response.jwt, // Pass JWT as session_id
                                axiom_jwt: None,
                                message: Some("Please check your email for the 2FA code".to_string()),
                                user_data: None,
                            })
                        } else {
                            Ok(AxiomAuthResponse {
                                success: false,
                                next_step: None,
                                session_id: None,
                                axiom_jwt: None,
                                message: step1_response.message.or_else(|| Some("Authentication failed".to_string())),
                                user_data: None,
                            })
                        }
                    }
                    Err(e) => {
                        error!("🔐 AXIOM: Login step 1 failed: {}", e);
                        Ok(AxiomAuthResponse {
                            success: false,
                            next_step: None,
                            session_id: None,
                            axiom_jwt: None,
                            message: Some(format!("Login failed: {}", e)),
                            user_data: None,
                        })
                    }
                }
            }
            "2fa" => {
                let jwt = request.session_id.ok_or("JWT is required for 2FA step")?;
                
                // If OTP code is not provided, automatically retrieve it from Gmail
                let otp_code = match request.otp_code {
                    Some(code) if !code.is_empty() => {
                        info!("🔐 AXIOM: Using provided OTP code");
                        code
                    }
                    _ => {
                        info!("🔐 AXIOM: No OTP code provided, retrieving from Gmail");
                        let _user_email = request.email.clone().unwrap_or_else(|| "the.last.tajae@gmail.com".to_string());
                        
                        // For now, return a helpful message that user needs to manually provide 2FA code
                        // TODO: Implement Gmail integration when backend Handler trait issue is resolved
                        return Ok(AxiomAuthResponse {
                            success: false,
                            next_step: None,
                            session_id: Some(jwt),
                            axiom_jwt: None,
                            message: Some("Please check your email for the Axiom 2FA code. Gmail integration is temporarily unavailable.".to_string()),
                            user_data: None,
                        });
                    }
                };
                // Require email and password here as well for legacy path
                let email = request.email.clone().unwrap_or_default();
                let password = request.password.unwrap_or_default();
                if email.is_empty() || password.is_empty() {
                    return Ok(AxiomAuthResponse {
                        success: false,
                        next_step: None,
                        session_id: Some(jwt),
                        axiom_jwt: None,
                        message: Some("Email and password are required for 2FA".to_string()),
                        user_data: None,
                    });
                }
                match self.login_step2(&jwt, &otp_code, &email, &password).await {
                    Ok(step2_response) => {
                        if step2_response.success {
                            Ok(AxiomAuthResponse {
                                success: true,
                                next_step: Some("complete".to_string()),
                                session_id: step2_response.access_token,
                                axiom_jwt: None,
                                message: Some("Authentication successful!".to_string()),
                                user_data: Some(AxiomUserData {
                                    email: email,
                                    name: "Axiom Trader".to_string(),
                                    account_id: step2_response.user_id.unwrap_or_else(|| "AXIOM_LIVE".to_string()),
                                    trading_enabled: true,
                                }),
                            })
                        } else {
                            Ok(AxiomAuthResponse {
                                success: false,
                                next_step: None,
                                session_id: Some(jwt),
                                axiom_jwt: None,
                                message: step2_response.message.or_else(|| Some("2FA verification failed".to_string())),
                                user_data: None,
                            })
                        }
                    }
                    Err(e) => {
                        error!("🔐 AXIOM: Login step 2 failed: {}", e);
                        Ok(AxiomAuthResponse {
                            success: false,
                            next_step: None,
                            session_id: Some(jwt),
                            axiom_jwt: None,
                            message: Some(format!("2FA verification failed: {}", e)),
                            user_data: None,
                        })
                    }
                }
            }
            _ => {
                Err(format!("Unknown authentication step: {}", request.step))
            }
        }
    }
}

impl Default for AxiomService {
    fn default() -> Self {
        Self::new()
    }
}