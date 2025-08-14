use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use worker::*;
use wasm_bindgen::JsValue;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE as BASE64_URL_SAFE;

#[derive(Debug, Clone)]
pub struct GmailService {
    access_token: String,
}

#[derive(Debug, Deserialize)]
struct GmailMessage {
    id: String,
    #[serde(rename = "threadId")]
    thread_id: String,
}

#[derive(Debug, Deserialize)]
struct GmailMessageList {
    messages: Option<Vec<GmailMessage>>,
}

#[derive(Debug, Deserialize)]
struct MessagePayload {
    headers: Vec<MessageHeader>,
    body: Option<MessageBody>,
    parts: Option<Vec<MessagePart>>,
}

#[derive(Debug, Deserialize)]
struct MessageHeader {
    name: String,
    value: String,
}

#[derive(Debug, Deserialize)]
struct MessageBody {
    data: Option<String>,
}

#[derive(Debug, Deserialize)]
struct MessagePart {
    #[serde(rename = "mimeType")]
    mime_type: String,
    body: Option<MessageBody>,
}

#[derive(Debug, Deserialize)]
struct FullMessage {
    id: String,
    payload: MessagePayload,
}

impl GmailService {
    /// Create a new Gmail service with an access token
    /// In production, you'd get this token through OAuth2 flow
    pub async fn new(access_token: String) -> Result<Self> {
        Ok(Self { access_token })
    }

    /// Search for Axiom 2FA code email sent in the last 5 minutes.
    /// Gmail query design based on official filtering syntax reference:
    ///   - newer_than:5m restricts to last 5 minutes (more concise than after:unix_ts)
    ///   - from:(no-reply@axiom.trade OR axiom) matches explicit address or display name containing 'Axiom'
    ///   - phrase matches target likely body/subject fragments.
    ///   - label:inbox biases toward primary received mail.
    /// We attempt a primary query then a looser fallback if no code found.
    pub async fn get_axiom_2fa_code(&self, _user_email: &str) -> Result<Option<String>> {
        console_log!("📧 GMAIL 2FA: Begin lookup (<=5m window)");
        let primary_query = "label:inbox newer_than:5m (from:(no-reply@axiom.trade OR axiom)) (\"security code\" OR \"Axiom security code\" OR \"code is:\")";
        console_log!("📧 GMAIL 2FA: Primary query => {}", primary_query);
        if let Some(code) = self.try_query_for_code(primary_query).await? { return Ok(Some(code)); }
        let fallback_query = "newer_than:5m (from:(no-reply@axiom.trade OR axiom)) (code OR verification OR \"is:\")";
        console_log!("📧 GMAIL 2FA: Fallback query => {}", fallback_query);
        if let Some(code) = self.try_query_for_code(fallback_query).await? { return Ok(Some(code)); }
        console_log!("📧 GMAIL 2FA: No code found after both queries");
        Ok(None)
    }

    async fn try_query_for_code(&self, query: &str) -> Result<Option<String>> {
        console_log!("📧 GMAIL 2FA: Executing query => {}", query);
        let messages = self.search_messages(query).await?;
        match messages {
            None => { console_log!("📧 GMAIL 2FA: No messages returned (None)"); Ok(None) }
            Some(msgs) => {
                console_log!("📧 GMAIL 2FA: {} messages returned (limit 8 processed)", msgs.len());
                for (i, msg) in msgs.iter().take(8).enumerate() {
                    console_log!("📧 GMAIL 2FA: Fetching message {} id={} thread={}", i, msg.id, msg.thread_id);
                    match self.get_message(&msg.id).await {
                        Ok(full) => {
                            if let Some(subj) = Self::header_value(&full, "Subject") { console_log!("📧 GMAIL 2FA:   Subject: {}", subj); }
                            if let Some(from) = Self::header_value(&full, "From") { console_log!("📧 GMAIL 2FA:   From: {}", from); }
                            if let Some(date) = Self::header_value(&full, "Date") { console_log!("📧 GMAIL 2FA:   Date: {}", date); }
                            let body_preview = self.get_message_body(&full).map(|b| sanitize_log_snippet(&b)).unwrap_or_else(|| "<no body>".into());
                            console_log!("📧 GMAIL 2FA:   Body preview: {}", body_preview);
                            match self.extract_2fa_code(&full) {
                                Some(code) => {
                                    console_log!("📧 GMAIL 2FA:   ✅ Candidate code: {}", code);
                                    if code.len() == 6 && code.chars().all(|c| c.is_ascii_digit()) {
                                        console_log!("📧 GMAIL 2FA:   ✅ Accepted 6-digit code");
                                        return Ok(Some(code));
                                    } else {
                                        console_log!("📧 GMAIL 2FA:   ❌ Rejected (not 6 digits numeric)");
                                    }
                                }
                                None => console_log!("📧 GMAIL 2FA:   No code pattern matched"),
                            }
                        }
                        Err(e) => console_log!("📧 GMAIL 2FA:   ⚠️ Failed to fetch message {}: {:?}", msg.id, e),
                    }
                }
                Ok(None)
            }
        }
    }

    /// Search Gmail messages with a query
    async fn search_messages(&self, query: &str) -> Result<Option<Vec<GmailMessage>>> {
        let url = format!(
            "https://gmail.googleapis.com/gmail/v1/users/me/messages?q={}&maxResults=10",
            urlencoding::encode(query)
        );
        console_log!("📧 GMAIL 2FA: search_messages URL={}", url);

        let headers = Headers::new();
        headers.set("Authorization", &format!("Bearer {}", self.access_token))?;

        let mut request_init = RequestInit::new();
        request_init.with_method(Method::Get).with_headers(headers);

        let request = Request::new_with_init(&url, &request_init)?;
        let mut response = Fetch::Request(request).send().await?;

        let status = response.status_code();
        console_log!("📧 GMAIL 2FA: search_messages status={}", status);
        if status == 200 {
            let text = response.text().await?;
            console_log!("📧 GMAIL 2FA: search_messages raw length={} bytes", text.len());
            let result: GmailMessageList = serde_json::from_str(&text)
                .map_err(|e| Error::RustError(format!("Failed to parse Gmail response: {}", e)))?;
            if let Some(ref msgs) = result.messages { console_log!("📧 GMAIL 2FA: Parsed {} message ids", msgs.len()); } else { console_log!("📧 GMAIL 2FA: No messages array in response"); }
            Ok(result.messages)
        } else {
            let err_body = response.text().await.unwrap_or_default();
            let snippet = sanitize_log_snippet(&err_body);
            console_log!("📧 GMAIL 2FA: search_messages error body={}...", snippet);
            if status == 403 {
                // Common misconfiguration: Gmail API not enabled in Google Cloud project
                if snippet.contains("has not been used") || snippet.to_lowercase().contains("disabled") {
                    console_log!("⚠️ GMAIL 2FA: Gmail API disabled or never enabled for this Google Cloud project. Enable at: https://console.cloud.google.com/apis/library/gmail.googleapis.com then re-authorize the user (consent screen) to grant gmail.readonly.");
                } else if snippet.to_lowercase().contains("insufficient permissions") {
                    console_log!("⚠️ GMAIL 2FA: Access token missing gmail.readonly scope. Ensure authorization URL includes it and user granted consent. Re-run OAuth flow.");
                }
            }
            Ok(None)
        }
    }

    /// Get a specific Gmail message by ID
    async fn get_message(&self, message_id: &str) -> Result<FullMessage> {
        let url = format!(
            "https://gmail.googleapis.com/gmail/v1/users/me/messages/{}",
            message_id
        );

        let headers = Headers::new();
        headers.set("Authorization", &format!("Bearer {}", self.access_token))?;

        let mut request_init = RequestInit::new();
        request_init.with_method(Method::Get).with_headers(headers);

        let request = Request::new_with_init(&url, &request_init)?;
        let mut response = Fetch::Request(request).send().await?;

        let status = response.status_code();
        if status == 200 {
            let text = response.text().await?;
            let message: FullMessage = serde_json::from_str(&text)
                .map_err(|e| Error::RustError(format!("Failed to parse message: {}", e)))?;
            Ok(message)
        } else {
            let err_body = response.text().await.unwrap_or_default();
            console_log!("📧 GMAIL 2FA: get_message failed status={} body={}...", status, sanitize_log_snippet(&err_body));
            Err(Error::RustError(format!("Failed to get message: {}", status)))
        }
    }

    /// Extract Axiom 2FA code from email message.
    /// Priority order:
    ///  1. Explicit pattern: "Your Axiom security code is: 123456" / "security code is: 123456"
    ///  2. Generic "is: 123456"
    ///  3. Other fallback patterns (e.g., "123456 is your", "code: 123456")
    fn extract_2fa_code(&self, message: &FullMessage) -> Option<String> {
    let raw_body = match self.get_message_body(message) { Some(b) => b, None => { console_log!("📧 GMAIL 2FA: extract_2fa_code no body" ); return None; } };
    let body_text = Self::strip_html(&raw_body).replace('\r', "");
    console_log!("📧 GMAIL 2FA: extract_2fa_code body_len={}", body_text.len());
        // Ordered regex list (first match wins)
        let ordered_patterns = [
            r"Your\s+Axiom\s+security\s+code\s+is:\s*(\d{6})"
        ];
        for pat in ordered_patterns.iter() {
            if let Ok(re) = regex::Regex::new(pat) {
                if let Some(cap) = re.captures(&body_text) { let c = cap.get(1)?.as_str().to_string(); console_log!("📧 GMAIL 2FA: Pattern '{}' matched {}", pat, c); return Some(c); }
            }
        }
        None
    }

    /// Extract text body from Gmail message
    fn get_message_body(&self, message: &FullMessage) -> Option<String> {
        // Check direct body
        if let Some(ref body) = message.payload.body {
            if let Some(ref data) = body.data {
                if let Ok(decoded) = BASE64_URL_SAFE.decode(data) {
                    if let Ok(text) = String::from_utf8(decoded) {
                        return Some(text);
                    }
                }
            }
        }

        // Check parts for text/plain or text/html
        if let Some(ref parts) = message.payload.parts {
            for part in parts {
                if part.mime_type == "text/plain" || part.mime_type == "text/html" {
                    if let Some(ref body) = part.body {
                        if let Some(ref data) = body.data {
                            if let Ok(decoded) = BASE64_URL_SAFE.decode(data) {
                                if let Ok(text) = String::from_utf8(decoded) {
                                    return Some(text);
                                }
                            }
                        }
                    }
                }
            }
        }

        None
    }

    /// Naive HTML stripper (sufficient for extracting simple code lines)
    fn strip_html(input: &str) -> String {
        if input.contains('<') && input.contains('>') {
            // remove tags
            let no_tags = regex::Regex::new(r"<[^>]+>").ok().map(|re| re.replace_all(input, " ").to_string()).unwrap_or_else(|| input.to_string());
            // collapse whitespace
            regex::Regex::new(r"\s+").ok().map(|re| re.replace_all(&no_tags, " ").to_string()).unwrap_or(no_tags)
        } else { input.to_string() }
    }
}

/// Gmail OAuth2 token management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GmailOAuth {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
}

impl GmailOAuth {
    /// Check if token is expired
    pub fn is_expired(&self) -> bool {
        Utc::now() >= self.expires_at
    }

    /// Refresh the access token using refresh token
    pub async fn refresh(&mut self, client_id: &str, client_secret: &str) -> Result<()> {
        let url = "https://oauth2.googleapis.com/token";

        let body = serde_json::json!({
            "client_id": client_id,
            "client_secret": client_secret,
            "refresh_token": self.refresh_token,
            "grant_type": "refresh_token"
        });

        let headers = Headers::new();
        headers.set("Content-Type", "application/json")?;

        let mut request_init = RequestInit::new();
        request_init
            .with_method(Method::Post)
            .with_headers(headers)
            .with_body(Some(JsValue::from_str(&body.to_string())));

        let request = Request::new_with_init(url, &request_init)?;
        let mut response = Fetch::Request(request).send().await?;

        if response.status_code() == 200 {
            let text = response.text().await?;
            let token_response: serde_json::Value = serde_json::from_str(&text)
                .map_err(|e| Error::RustError(format!("Failed to parse token response: {}", e)))?;

            if let Some(access_token) = token_response["access_token"].as_str() {
                self.access_token = access_token.to_string();
                if let Some(expires_in) = token_response["expires_in"].as_i64() {
                    self.expires_at = Utc::now() + Duration::seconds(expires_in);
                }
                Ok(())
            } else {
                Err(Error::RustError("No access token in response".to_string()))
            }
        } else {
            Err(Error::RustError(format!(
                "Failed to refresh token: {}",
                response.status_code()
            )))
        }
    }
}

pub(crate) fn sanitize_log_snippet(s: &str) -> String {
    let mut out = s.chars().take(160).collect::<String>();
    if s.len() > 160 { out.push_str("…"); }
    if let Ok(re) = regex::Regex::new(r"(\d{6,})") { out = re.replace_all(&out, "<redacted>").to_string(); }
    out
}

impl GmailService {
    fn header_value(full: &FullMessage, name: &str) -> Option<String> {
        full.payload.headers.iter().find(|h| h.name.eq_ignore_ascii_case(name)).map(|h| h.value.clone())
    }
}