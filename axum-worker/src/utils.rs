use worker::{console_log, Date, Request};

pub fn set_panic_hook() {
    // Set panic hook for better error reporting in development
    // Note: console_error_panic_hook is not currently configured as a feature
    // but this function is kept for future use
}

/// Log incoming requests with location and region information
pub fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().map(|cf| cf.coordinates().unwrap_or_default()).unwrap_or_default(),
        req.cf().and_then(|cf| cf.region()).unwrap_or_else(|| "unknown region".into())
    );
}