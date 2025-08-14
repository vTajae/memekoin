use crate::service::axiom::{AxiomService, LoginStep1Response, LoginStep2Response};

/// Axiom client wrapper that uses axiomtrade-rs when the `axiomtrade` feature is enabled
/// and falls back to our WASM-compatible service otherwise.
#[derive(Clone)]
pub struct AxiomClient {
    service: AxiomService,
}

impl AxiomClient {
    pub fn new() -> Self { Self { service: AxiomService::new() } }

    /// Step 1: trigger OTP and retrieve jwt token
    pub async fn login_step1(&mut self, email: &str, password: &str) -> worker::Result<LoginStep1Response> {
        // On native with `axiomtrade` feature, we can still use service which is now aligned
        self.service.login_step1(email, password).await
    }

    /// Step 2: complete auth using jwt + otp
    pub async fn login_step2(&mut self, jwt: &str, otp_code: &str, email: &str, base64_password: &str) -> worker::Result<LoginStep2Response> {
        self.service.login_step2(jwt, otp_code, email, base64_password).await
    }

    /// Optional: one-shot login using the axiomtrade SDK on native targets.
    #[cfg(feature = "axiomtrade")]
    pub async fn login_via_sdk(&mut self, email: &str, password: &str, otp_code: Option<String>) -> worker::Result<LoginStep2Response> {
        self.service.login_with_axiomtrade_sdk(email, password, otp_code).await
    }
}
