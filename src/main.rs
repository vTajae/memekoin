#[cfg(feature = "ssr")]
pub fn main() {
    // no main function needed for Cloudflare Workers
    // see lib.rs for the fetch event handler
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // see lib.rs for hydration function instead
}