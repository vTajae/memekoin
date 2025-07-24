/// Format a number as currency with the specified symbol
pub fn format_currency_with_symbol(amount: f64, symbol: &str) -> String {
    format!("{}{:.2}", symbol, amount)
}

/// Format a price with appropriate decimal places for trading
pub fn format_price(price: f64) -> String {
    if price >= 1000.0 {
        format!("{:.2}", price)
    } else if price >= 1.0 {
        format!("{:.4}", price)
    } else if price >= 0.01 {
        format!("{:.6}", price)
    } else {
        format!("{:.8}", price)
    }
}

/// Format a currency amount (defaults to USD)
pub fn format_currency(amount: f64) -> String {
    if amount.abs() >= 1_000_000.0 {
        format!("${:.2}M", amount / 1_000_000.0)
    } else if amount.abs() >= 1_000.0 {
        format!("${:.2}K", amount / 1_000.0)
    } else {
        format!("${:.2}", amount)
    }
}

/// Format trading volume with appropriate units
pub fn format_volume(volume: f64) -> String {
    if volume >= 1_000_000_000.0 {
        format!("{:.2}B", volume / 1_000_000_000.0)
    } else if volume >= 1_000_000.0 {
        format!("{:.2}M", volume / 1_000_000.0)
    } else if volume >= 1_000.0 {
        format!("{:.2}K", volume / 1_000.0)
    } else {
        format!("{:.2}", volume)
    }
}

/// Format a number with thousands separators
pub fn format_number(num: i64) -> String {
    let num_str = num.to_string();
    let chars: Vec<char> = num_str.chars().collect();
    let mut result = String::new();
    
    for (i, ch) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i) % 3 == 0 {
            result.push(',');
        }
        result.push(*ch);
    }
    
    result
}

/// Format a percentage with the specified decimal places
pub fn format_percentage_with_decimals(value: f64, decimal_places: usize) -> String {
    format!("{:.1$}%", value * 100.0, decimal_places)
}

/// Format a percentage (defaults to 2 decimal places)
pub fn format_percentage(value: f64) -> String {
    format!("{:.2}%", value)
}

/// Format a file size in human-readable format
pub fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

/// Format a duration in human-readable format
pub fn format_duration(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;
    
    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, secs)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, secs)
    } else {
        format!("{}s", secs)
    }
}

/// Truncate text to a specified length with ellipsis
pub fn truncate_text(text: &str, max_length: usize) -> String {
    if text.len() <= max_length {
        text.to_string()
    } else {
        format!("{}...", &text[..max_length.saturating_sub(3)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_currency() {
        assert_eq!(format_currency(1234.56, "$"), "$1234.56");
        assert_eq!(format_currency(0.99, "€"), "€0.99");
    }

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(1234), "1,234");
        assert_eq!(format_number(1234567), "1,234,567");
        assert_eq!(format_number(123), "123");
    }

    #[test]
    fn test_format_percentage() {
        assert_eq!(format_percentage(0.1234, 2), "12.34%");
        assert_eq!(format_percentage(0.5, 1), "50.0%");
    }

    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(1024), "1.0 KB");
        assert_eq!(format_file_size(1536), "1.5 KB");
        assert_eq!(format_file_size(1048576), "1.0 MB");
    }

    #[test]
    fn test_truncate_text() {
        assert_eq!(truncate_text("Hello, World!", 10), "Hello, ...");
        assert_eq!(truncate_text("Short", 10), "Short");
    }
}
