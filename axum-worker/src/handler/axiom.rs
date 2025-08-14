//! Axiom Trade Authentication Handler
//! Handles authentication flow with 2FA via Gmail integration

use crate::state::AppState;
use axum::{
    Json,
    body::{self, Body},
    extract::State,
    http::Request,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use worker::console_log;
// use crate::repository::session::UserSessionData; // no direct use after refactor
use serde_json;

// use crate::utils::error::AppError;

/// Request for Axiom authentication with Gmail 2FA
#[derive(Deserialize)]
pub struct AxiomAuthRequest {
    pub step: String,
    pub email: Option<String>,
    // Accept raw password (legacy) or provided base64password (preferred)
    #[serde(default)]
    pub password: Option<String>,
    #[serde(rename = "base64password", alias = "b64Password")]
    #[serde(default)]
    pub base64_password: Option<String>,
    // JWT can come as body field (legacy) or header; accept alias "jwt"
    #[serde(alias = "jwt")]
    #[serde(default)]
    pub session_id: Option<String>,
    // OTP code; accept alias "code"
    #[serde(alias = "code")]
    #[serde(default)]
    pub otp_code: Option<String>,
}

/// Response for Axiom authentication
#[derive(Serialize)]
pub struct AxiomAuthResponse {
    pub success: bool,
    pub next_step: Option<String>,
    pub session_id: Option<String>,
    pub axiom_jwt: Option<String>,
    pub message: Option<String>,
    pub user_data: Option<AxiomUserData>,
}

#[derive(Serialize, Clone)]
pub struct AxiomUserData {
    pub email: String,
    pub name: String,
    pub account_id: String,
    pub trading_enabled: bool,
}

/// Gmail 2FA request structure
#[derive(Deserialize)]
pub struct Gmail2FARequest {
    pub user_email: String,
    pub access_token: Option<String>, // Google OAuth access token (bearer)
}

/// Gmail 2FA response structure
#[derive(Serialize)]
pub struct Gmail2FAResponse {
    pub success: bool,
    pub message: Option<String>,
    pub otp_code: Option<String>,
}

/// Simple API response structure for JSON responses
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

/// Handler for getting Axiom 2FA code from Gmail (non-Send; avoid debug_handler)
pub async fn get_axiom_2fa_from_gmail(
    State(app_state): State<AppState>,
    headers: axum::http::HeaderMap,
    Json(request): Json<Gmail2FARequest>,
) -> Json<Gmail2FAResponse> {
    console_log!(
        "📧 Manual Gmail 2FA fetch attempt for: {}",
        request.user_email
    );
    use futures::channel::oneshot;
    let (tx, rx) = oneshot::channel();
    let email = request.user_email.clone();
    let provided_token = request.access_token.clone();
    let db = app_state.database();

    // Extract session cookie from headers
    let session_cookie = headers
        .get(axum::http::header::COOKIE)
        .and_then(|value| value.to_str().ok())
        .and_then(|cookie_str| {
            cookie_str.split(';').find_map(|cookie| {
                let cookie = cookie.trim();
                if cookie.starts_with("session_id=") {
                    cookie.split('=').nth(1).map(|s| s.trim().to_string())
                } else {
                    None
                }
            })
        });

    wasm_bindgen_futures::spawn_local(async move {
        let session_service = crate::service::session::SessionService::new(db.clone());

        // Try to get user_id from session cookie for token refresh capability
        let mut token_opt = None;

        if let Some(ref session_cookie_value) = session_cookie {
            if let Some(user_id) = session_service.parse_user_id_from_cookie(session_cookie_value) {
                console_log!(
                    "📧 Gmail 2FA: Using user_id from session for token lookup with refresh: {}",
                    user_id
                );
                // Use the new method with refresh logic
                match session_service
                    .get_latest_google_access_token_by_user_id(user_id)
                    .await
                {
                    Ok(tok) => {
                        if tok.is_none() {
                            console_log!(
                                "ℹ️ Gmail 2FA: No valid token found for user_id {} (even after refresh attempt)",
                                user_id
                            );
                        } else {
                            console_log!("✅ Gmail 2FA: Token obtained for user_id {}", user_id);
                        }
                        token_opt = tok;
                    }
                    Err(e) => {
                        console_log!("⚠️ Gmail 2FA token lookup (user_id) error: {}", e);
                    }
                }
            }
        }

        // Fallback to email-based lookup (without refresh)
        if token_opt.is_none() {
            console_log!("📧 Gmail 2FA: Falling back to email-based token lookup (no refresh)");
            match session_service.get_latest_google_access_token(&email).await {
                Ok(tok) => {
                    if tok.is_none() {
                        console_log!("ℹ️ Gmail 2FA: No token by email {}", email);
                    }
                    token_opt = tok;
                }
                Err(e) => {
                    console_log!("⚠️ Gmail 2FA token lookup (email) error: {}", e);
                }
            };
        }

        // 2. Fallback: account_sessions JSON
        if token_opt.is_none() {
            if let Ok(Some(sess)) = session_service.get_latest_session_by_email(&email).await {
                if let Some(tok) = sess.access_token {
                    console_log!("ℹ️ Gmail 2FA: Using token from account_sessions JSON");
                    token_opt = Some(tok);
                }
            }
        }

        // 3. Fallback: provided access_token field
        if token_opt.is_none() && provided_token.is_some() {
            console_log!("ℹ️ Gmail 2FA: Using provided token override");
            token_opt = provided_token.clone();
        }

        let resp = if let Some(token) = token_opt {
            match crate::service::gmail::GmailService::new(token).await {
                Ok(gmail) => match gmail.get_axiom_2fa_code(&email).await {
                    Ok(Some(code)) => Gmail2FAResponse {
                        success: true,
                        message: Some("Code retrieved".into()),
                        otp_code: Some(code),
                    },
                    Ok(None) => Gmail2FAResponse {
                        success: false,
                        message: Some("No recent Axiom code email found".into()),
                        otp_code: None,
                    },
                    Err(e) => Gmail2FAResponse {
                        success: false,
                        message: Some(format!("Gmail fetch error: {}", e)),
                        otp_code: None,
                    },
                },
                Err(e) => Gmail2FAResponse {
                    success: false,
                    message: Some(format!("Init Gmail service failed: {}", e)),
                    otp_code: None,
                },
            }
        } else {
            Gmail2FAResponse {
                success: false,
                message: Some(
                    "No stored Google token found for user; supply access_token to endpoint".into(),
                ),
                otp_code: None,
            }
        };
        let _ = tx.send(resp);
    });
    let result = rx.await.unwrap_or(Gmail2FAResponse {
        success: false,
        message: Some("Internal channel error".into()),
        otp_code: None,
    });
    Json(result)
}

/// Main handler for Axiom authentication flow
#[axum::debug_handler]
pub async fn handle_axiom_auth(
    State(app_state): State<AppState>,
    req: Request<Body>,
) -> Json<AxiomAuthResponse> {
    // Extract JSON body manually because we also need headers for session cookie
    let (parts, body) = req.into_parts();
    // Read request body into bytes (limit ~64KB)
    let whole = body::to_bytes(body, 64 * 1024).await.unwrap_or_default();
    let request: AxiomAuthRequest = serde_json::from_slice(&whole).unwrap_or(AxiomAuthRequest {
        step: "".into(),
        email: None,
        password: None,
        base64_password: None,
        session_id: None,
        otp_code: None,
    });

    // Helper mask utilities
    fn mask_password(pw_opt: &Option<String>) -> String {
        pw_opt
            .as_ref()
            .map(|pw| "*".repeat(std::cmp::min(8, pw.len())))
            .unwrap_or("<none>".into())
    }
    fn mask_token(tok_opt: &Option<String>) -> String {
        tok_opt
            .as_ref()
            .map(|t| {
                if t.len() <= 10 {
                    format!("{}", t)
                } else {
                    format!("{}...{} (len={})", &t[..6], &t[t.len() - 4..], t.len())
                }
            })
            .unwrap_or("<none>".into())
    }

    console_log!(
        "🔐 AXIOM: Incoming request step='{}' email='{}' password='{}' session_id='{}' otp_supplied={}",
        request.step,
        request.email.clone().unwrap_or_default(),
        mask_password(&request.password),
        request.session_id.clone().unwrap_or_default(),
        request.otp_code.is_some()
    );
    // Grab existing web session cookie (not the Axiom jwt) if present
    let web_session_cookie = parts
        .headers
        .get(axum::http::header::COOKIE)
        .and_then(|hv| hv.to_str().ok())
        .and_then(|cookie_str| {
            cookie_str.split(';').find_map(|c| {
                let c = c.trim();
                if c.starts_with("session_id=") {
                    c.split('=').nth(1).map(|v| v.to_string())
                } else {
                    None
                }
            })
        });
    // Prefer OTP JWT from header if supplied (frontend may send this explicitly)
    let header_axiom_jwt = parts
        .headers
        .get("x-axiom-otp-jwt")
        .and_then(|hv| hv.to_str().ok())
        .map(|s| s.to_string());
    use crate::client::AxiomClient;
    // (Future) Import session repository when persistence is fully wired: use crate::repository::session::SessionRepository;
    use futures::channel::oneshot;
    console_log!(
        "🔐 AXIOM: Handling auth step (Send-bridge): {} (enforcing mandatory 2FA)",
        request.step
    );

    // Bridge non-Send worker::Fetch future via spawn_local + oneshot
    let (tx, rx) = oneshot::channel();
    let req_clone = request; // move into closure
    // Clone resources needed inside non-Send closure
    let session_service = crate::service::session::SessionService::new(app_state.database());
    let web_session_cookie_cloned = web_session_cookie.clone();
    let header_axiom_jwt_cloned = header_axiom_jwt.clone();

    wasm_bindgen_futures::spawn_local(async move {
        let build_error = |msg: &str| AxiomAuthResponse {
            success: false,
            next_step: None,
            session_id: None,
            axiom_jwt: None,
            message: Some(msg.to_string()),
            user_data: None,
        };

        // Infer step if omitted
        let inferred_step = if req_clone.step.is_empty()
            && (req_clone.otp_code.is_some() || req_clone.base64_password.is_some())
        {
            "2fa".to_string()
        } else {
            req_clone.step.clone()
        };

        let result: AxiomAuthResponse = if inferred_step == "email" {
            let email = if let Some(e) = req_clone.email.clone() { e } else { let _ = tx.send(build_error("Email is required")); return; };
            let password = if let Some(p) = req_clone.password.clone() { p } else { let _ = tx.send(build_error("Password is required")); return; };
            if email.is_empty() || password.is_empty() {
                build_error("Email and password are required")
            } else {
                let mut axiom = AxiomClient::new();
                match axiom.login_step1(&email, &password).await {
                    Ok(login_resp) => {
                        if login_resp.success {
                            AxiomAuthResponse {
                                success: true,
                                next_step: Some("2fa".into()),
                                session_id: web_session_cookie_cloned.clone(),
                                axiom_jwt: login_resp.jwt,
                                message: Some("Enter 2FA code".into()),
                                user_data: None,
                            }
                        } else {
                            AxiomAuthResponse {
                                success: false,
                                next_step: None,
                                session_id: web_session_cookie_cloned.clone(),
                                axiom_jwt: None,
                                message: login_resp.message.or(Some("Authentication failed".into())),
                                user_data: None,
                            }
                        }
                    }
                    Err(e) => AxiomAuthResponse {
                        success: false,
                        next_step: None,
                        session_id: web_session_cookie_cloned.clone(),
                        axiom_jwt: None,
                        message: Some(format!("Login step 1 failed: {}", e)),
                        user_data: None,
                    }
                }
            }
        } else if inferred_step == "2fa" {
            // Determine email first (for both SDK and fallback flows)
            let email = if let Some(e) = req_clone.email.clone().filter(|e| !e.is_empty()) { e } else {
                if let Some(session_cookie_id) = web_session_cookie_cloned.clone() {
                    match session_service.get_user_session_data(&session_cookie_id).await {
                        Ok(Some(data)) => data.user_email,
                        _ => { let _ = tx.send(build_error("Email is required for 2FA")); return; }
                    }
                } else { let _ = tx.send(build_error("Email is required for 2FA")); return; }
            };

            // Native-only fast path using axiomtrade-rs SDK (optional feature). This can auto-fetch OTP.
            #[cfg(all(feature = "axiomtrade", not(target_arch = "wasm32")))]
            if req_clone.password.is_some() {
                let mut axiom = AxiomClient::new();
                match axiom
                    .login_via_sdk(&email, &req_clone.password.clone().unwrap(), req_clone.otp_code.clone())
                    .await
                {
                    Ok(step2_resp) => {
                        if step2_resp.success {
                            if let Some(session_cookie_id) = web_session_cookie_cloned.clone() {
                                let _ = session_service.update_axiom_tokens(
                                    &session_cookie_id,
                                    step2_resp.access_token.clone(),
                                    step2_resp.refresh_token.clone(),
                                    step2_resp.user_id.clone(),
                                    Utc::now(),
                                ).await;
                            }
                            let _ = tx.send(AxiomAuthResponse {
                                success: true,
                                next_step: Some("complete".into()),
                                session_id: web_session_cookie_cloned.clone(),
                                axiom_jwt: None,
                                message: step2_resp.message.or(Some("Authentication successful!".into())),
                                user_data: Some(AxiomUserData {
                                    email: email.clone(),
                                    name: "Axiom Trader".into(),
                                    account_id: step2_resp.user_id.clone().unwrap_or_else(|| "AXIOM_LIVE".into()),
                                    trading_enabled: true,
                                }),
                            });
                            return;
                        }
                    }
                    Err(_e) => {
                        // Fall through to the regular fallback path below
                    }
                }
            }

            // Fallback WASM-compatible flow using JWT + OTP
            let jwt = if let Some(h) = header_axiom_jwt_cloned.clone() { h } else if let Some(b) = req_clone.session_id.clone() { b } else { let _ = tx.send(build_error("JWT is required for 2FA")); return; };
            // OTP code required in fallback
            let otp = if let Some(o) = req_clone.otp_code.clone() { o } else { let _ = tx.send(build_error("OTP code is required for 2FA")); return; };
            // Use provided b64 password or derive from raw (same as step1)
            let b64_pw = if let Some(b) = req_clone.base64_password.clone() { b } else if let Some(raw) = req_clone.password.clone() { crate::service::axiom::AxiomService::hash_password(&raw) } else { let _ = tx.send(build_error("Password is required for 2FA")); return; };
            let mut axiom = AxiomClient::new();
            match axiom.login_step2(&jwt, &otp, &email, &b64_pw).await {
                Ok(step2_resp) => {
                    if step2_resp.success {
                        if let Some(session_cookie_id) = web_session_cookie_cloned.clone() {
                            let _ = session_service.update_axiom_tokens(&session_cookie_id, step2_resp.access_token.clone(), step2_resp.refresh_token.clone(), step2_resp.user_id.clone(), Utc::now()).await;
                        }
                        AxiomAuthResponse {
                            success: true,
                            next_step: Some("complete".into()),
                            session_id: web_session_cookie_cloned.clone(),
                            axiom_jwt: None,
                            message: step2_resp.message.or(Some("Authentication successful!".into())),
                            user_data: Some(AxiomUserData { email: email.clone(), name: "Axiom Trader".into(), account_id: step2_resp.user_id.clone().unwrap_or_else(|| "AXIOM_LIVE".into()), trading_enabled: true }),
                        }
                    } else {
                        // Wait and retry: fetch a fresh code via Gmail API after 10 seconds, then attempt again once
                        worker::console_log!("🔁 AXIOM 2FA: First attempt failed, waiting 10s to fetch a fresh OTP via Gmail...");
                        gloo_timers::future::sleep(std::time::Duration::from_secs(10)).await;

                        // Attempt to fetch a newer code using stored Gmail token
                        let fresh_code = {
                            let mut token_opt = None;
                            if let Some(session_cookie_id) = web_session_cookie_cloned.clone() {
                                if let Ok(Some(user_session)) = session_service.get_user_session_data(&session_cookie_id).await {
                                    if let Ok(user_uuid) = uuid::Uuid::parse_str(&user_session.user_id) {
                                        if let Ok(tok) = session_service.get_latest_google_access_token_by_user_id(user_uuid).await { token_opt = tok; }
                                    }
                                }
                            }
                            if token_opt.is_none() {
                                if let Ok(tok) = session_service.get_latest_google_access_token(&email).await { token_opt = tok; }
                            }
                            if let Some(token) = token_opt {
                                match crate::service::gmail::GmailService::new(token).await {
                                    Ok(gmail) => {
                                        // Try default strict window, then a single flexible fallback (<=4m window, <=300s age)
                                        match gmail.get_axiom_2fa_code(&email).await {
                                            Ok(Some(c)) => Some(c),
                                            Ok(None) => match gmail.get_axiom_2fa_code_within(300, 4).await { Ok(Some(c2)) => Some(c2), _ => None },
                                            Err(_) => None,
                                        }
                                    },
                                    Err(_) => None,
                                }
                            } else { None }
                        };

                        if let Some(new_code) = fresh_code.filter(|c| c != &otp) {
                            worker::console_log!("🔁 AXIOM 2FA: Retrying with freshly fetched OTP");
                            match axiom.login_step2(&jwt, &new_code, &email, &b64_pw).await {
                                Ok(step2b) if step2b.success => {
                                    if let Some(session_cookie_id) = web_session_cookie_cloned.clone() {
                                        let _ = session_service.update_axiom_tokens(&session_cookie_id, step2b.access_token.clone(), step2b.refresh_token.clone(), step2b.user_id.clone(), Utc::now()).await;
                                    }
                                    AxiomAuthResponse {
                                        success: true,
                                        next_step: Some("complete".into()),
                                        session_id: web_session_cookie_cloned.clone(),
                                        axiom_jwt: None,
                                        message: step2b.message.or(Some("Authentication successful!".into())),
                                        user_data: Some(AxiomUserData { email: email.clone(), name: "Axiom Trader".into(), account_id: step2b.user_id.clone().unwrap_or_else(|| "AXIOM_LIVE".into()), trading_enabled: true }),
                                    }
                                }
                                Ok(step2b) => {
                                    let message = match step2b.message.clone() { Some(m) if m.contains("Try again later") || m.contains("Rate limit") => Some("Try again later".to_string()), other => other };
                                    AxiomAuthResponse { success: false, next_step: None, session_id: web_session_cookie_cloned.clone(), axiom_jwt: Some(jwt), message: message.or(Some("2FA verification failed".to_string())), user_data: None }
                                }
                                Err(e2) => {
                                    let msg = e2.to_string();
                                    let friendly = if msg.contains("Rate limit exceeded") { "Try again later".to_string() } else { format!("2FA verification failed: {}", msg) };
                                    AxiomAuthResponse { success: false, next_step: None, session_id: web_session_cookie_cloned.clone(), axiom_jwt: Some(jwt), message: Some(friendly), user_data: None }
                                }
                            }
                        } else {
                            let message = match step2_resp.message.clone() { Some(m) if m.contains("Try again later") || m.contains("Rate limit") => Some("Try again later".to_string()), other => other };
                            AxiomAuthResponse { success: false, next_step: None, session_id: web_session_cookie_cloned.clone(), axiom_jwt: Some(jwt), message: message.or(Some("2FA verification failed".to_string())), user_data: None }
                        }
                    }
                }
                Err(e) => {
                    let msg = e.to_string();
                    let friendly = if msg.contains("Rate limit exceeded") { "Try again later".to_string() } else { format!("2FA verification failed: {}", msg) };
                    AxiomAuthResponse { success: false, next_step: None, session_id: web_session_cookie_cloned.clone(), axiom_jwt: Some(jwt), message: Some(friendly), user_data: None }
                }
            }
        } else {
            build_error(&format!("Unknown authentication step: {}", req_clone.step))
        };
        let _ = tx.send(result);
    });

    let resp = rx.await.unwrap_or(AxiomAuthResponse {
        success: false,
        next_step: None,
        session_id: web_session_cookie.clone(),
        axiom_jwt: None,
        message: Some("Internal channel error".to_string()),
        user_data: None,
    });
    Json(resp)
}
