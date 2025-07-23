use regex::Regex;

/// Validate an email address
pub fn validate_email(email: &str) -> bool {
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    email_regex.is_match(email)
}

/// Validate a cryptocurrency wallet address (basic validation)
pub fn validate_wallet_address(address: &str) -> bool {
    // Basic validation for common wallet address formats
    if address.is_empty() || address.len() < 26 || address.len() > 62 {
        return false;
    }
    
    // Check for valid characters (alphanumeric)
    address.chars().all(|c| c.is_alphanumeric())
}

/// Validate a password with enterprise security requirements
pub fn validate_password(password: &str) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    
    if password.len() < 8 {
        errors.push("Password must be at least 8 characters long".to_string());
    }
    
    if password.len() > 128 {
        errors.push("Password must be less than 128 characters long".to_string());
    }
    
    if !password.chars().any(|c| c.is_lowercase()) {
        errors.push("Password must contain at least one lowercase letter".to_string());
    }
    
    if !password.chars().any(|c| c.is_uppercase()) {
        errors.push("Password must contain at least one uppercase letter".to_string());
    }
    
    if !password.chars().any(|c| c.is_numeric()) {
        errors.push("Password must contain at least one number".to_string());
    }
    
    if !password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c)) {
        errors.push("Password must contain at least one special character".to_string());
    }
    
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validate a username
pub fn validate_username(username: &str) -> Result<(), String> {
    if username.is_empty() {
        return Err("Username cannot be empty".to_string());
    }
    
    if username.len() < 3 {
        return Err("Username must be at least 3 characters long".to_string());
    }
    
    if username.len() > 30 {
        return Err("Username must be less than 30 characters long".to_string());
    }
    
    if !username.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
        return Err("Username can only contain letters, numbers, underscores, and hyphens".to_string());
    }
    
    if username.starts_with('_') || username.starts_with('-') {
        return Err("Username cannot start with an underscore or hyphen".to_string());
    }
    
    Ok(())
}

/// Validate a phone number (basic international format)
pub fn validate_phone_number(phone: &str) -> bool {
    let phone_regex = Regex::new(r"^\+?[1-9]\d{1,14}$").unwrap();
    let cleaned = phone.replace(&[' ', '-', '(', ')'][..], "");
    phone_regex.is_match(&cleaned)
}

/// Validate a URL
pub fn validate_url(url: &str) -> bool {
    let url_regex = Regex::new(r"^https?://[^\s/$.?#].[^\s]*$").unwrap();
    url_regex.is_match(url)
}

/// Validate a numeric amount (for financial transactions)
pub fn validate_amount(amount: &str) -> Result<f64, String> {
    match amount.parse::<f64>() {
        Ok(value) => {
            if value < 0.0 {
                Err("Amount cannot be negative".to_string())
            } else if value > 1_000_000_000.0 {
                Err("Amount is too large".to_string())
            } else {
                Ok(value)
            }
        }
        Err(_) => Err("Invalid amount format".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_email() {
        assert!(validate_email("user@example.com"));
        assert!(validate_email("test.email+tag@domain.co.uk"));
        assert!(!validate_email("invalid.email"));
        assert!(!validate_email("@domain.com"));
    }

    #[test]
    fn test_validate_password() {
        assert!(validate_password("SecurePass123!").is_ok());
        assert!(validate_password("weak").is_err());
        assert!(validate_password("NoNumbers!").is_err());
        assert!(validate_password("nonumbers123!").is_err());
    }

    #[test]
    fn test_validate_username() {
        assert!(validate_username("valid_user").is_ok());
        assert!(validate_username("user123").is_ok());
        assert!(validate_username("ab").is_err());
        assert!(validate_username("_invalid").is_err());
        assert!(validate_username("user@invalid").is_err());
    }

    #[test]
    fn test_validate_amount() {
        assert!(validate_amount("123.45").is_ok());
        assert!(validate_amount("0").is_ok());
        assert!(validate_amount("-10").is_err());
        assert!(validate_amount("invalid").is_err());
    }
}
