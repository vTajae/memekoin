At this point this application is broken. Use these references: 

This is a leptos fullstack rust cloudflare worker.

========= Architecture ===========
Index these references to future reference if there any wasm related errors:
https://gitlab.com/shellsort/fullstack-leptos-cloudflare-template
https://github.com/vTajae/rusty-worker
https://github.com/tokio-rs/axum/tree/main/examples/simple-router-wasm
https://www.geeksforgeeks.org/system-design/repository-design-pattern/

======     API     =======
https://docs.rs/axum-controller/latest/axum_controller/
https://docs.rs/axum/latest/axum/
https://docs.rs/tower-sessions/latest/tower_sessions/#cookie
[The cookie will be for the client side (leptos-wasm) WASM to interact with the backend (axum-worker]
https://docs.rs/tokio-postgres/latestaokio_postgres/
https://docs.rs/axum_session/latest/axum_session/config/struct.SessionConfig.html


=========   Google oAuth Analysis   ==============
https://gitlab.com/shellsort/fullstack-leptos-cloudflare-template

https://developers.google.com/identity/protocols/oauth2/web-server


Step 5: Exchange authorization code for refresh and access tokens
Sample Response From Token Exchange: 

{
  "access_token": "1/fFAGRNJru1FTz70BzhT3Zg",
  "expires_in": 3920,
  "token_type": "Bearer",
  "scope": "https://www.googleapis.com/auth/drive.metadata.readonly https://www.googleapis.com/auth/calendar.readonly",
  "refresh_token": "1//xEoDL4iW3cxlI7yDbSRFYNG01kVKM2C-259HOF2aQbI"
}


For the gmail api consider is information: 




===== Front End ====

Leptos Wasm

https://leptos-use.rs
https://leptos-use.rs/browser/use_cookie.html

