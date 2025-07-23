// Client-side functions that simulate server calls
// These will be replaced with real server functions once WASM memory issues are resolved

pub async fn say_hello(count: i32) -> Result<String, String> {
    // Reduced simulation delay for better performance
    #[cfg(target_arch = "wasm32")]
    {
        // Remove artificial delay to improve response time
        // In a real app, this would be an actual API call
    }

    let responses = [
        "Hello from client simulation! 🚀",
        "Simulated server response! ⚡",
        "Client-side function executed! ✅",
        "Mock API response delivered! 🏢",
        "Rust-powered simulation! 🦀",
    ];

    let response_index = (count as usize) % responses.len();
    let response = responses[response_index];

    Ok(format!("{} (Call #{count})", response))
}
