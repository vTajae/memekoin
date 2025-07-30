use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize};
use web_sys::{window, Storage};

// User info structure (matches backend)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub email: String,
}

// Login request structure
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

// Registration request structure
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

// Authentication response structure
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub success: bool,
    pub message: String,
    pub user: Option<UserInfo>,
    pub token: Option<String>,
}

// Authentication state
#[derive(Debug, Clone, PartialEq)]
pub enum AuthState {
    Loading,
    Authenticated(UserInfo),
    Unauthenticated,
}

// Local storage utilities
pub fn get_local_storage() -> Option<Storage> {
    window()?.local_storage().ok()?
}

pub fn save_token(token: &str) {
    if let Some(storage) = get_local_storage() {
        let _ = storage.set_item("auth_token", token);
    }
}

pub fn get_token() -> Option<String> {
    get_local_storage()?.get_item("auth_token").ok()?
}

pub fn remove_token() {
    if let Some(storage) = get_local_storage() {
        let _ = storage.remove_item("auth_token");
    }
}

pub fn save_user(user: &UserInfo) {
    if let Some(storage) = get_local_storage() {
        if let Ok(user_json) = serde_json::to_string(user) {
            let _ = storage.set_item("user_info", &user_json);
        }
    }
}

pub fn get_user() -> Option<UserInfo> {
    let storage = get_local_storage()?;
    let user_json = storage.get_item("user_info").ok()??;
    serde_json::from_str(&user_json).ok()
}

pub fn remove_user() {
    if let Some(storage) = get_local_storage() {
        let _ = storage.remove_item("user_info");
    }
}

// API utilities
pub async fn api_login(username: String, password: String) -> Result<AuthResponse, String> {
    let request = LoginRequest { username, password };
    
    let response = gloo_net::http::Request::post("/api/auth/login")
        .json(&request)
        .map_err(|e| format!("Failed to serialize request: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if response.ok() {
        response
            .json::<AuthResponse>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    } else {
        Err(format!("Login failed with status: {}", response.status()))
    }
}

pub async fn api_register(username: String, email: String, password: String) -> Result<AuthResponse, String> {
    let request = RegisterRequest { username, email, password };
    
    let response = gloo_net::http::Request::post("/api/auth/register")
        .json(&request)
        .map_err(|e| format!("Failed to serialize request: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if response.ok() {
        response
            .json::<AuthResponse>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    } else {
        Err(format!("Registration failed with status: {}", response.status()))
    }
}

pub async fn api_get_current_user() -> Result<UserInfo, String> {
    let token = get_token().ok_or("No token found")?;
    
    let response = gloo_net::http::Request::get("/api/auth/me")
        .header("Authorization", &format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if response.ok() {
        response
            .json::<UserInfo>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    } else {
        Err(format!("Failed to get user info with status: {}", response.status()))
    }
}

// Authentication context
#[derive(Clone, Copy)]
pub struct AuthContext {
    pub state: ReadSignal<AuthState>,
    pub set_state: WriteSignal<AuthState>,
}

pub fn provide_auth_context() -> AuthContext {
    let (state, set_state) = signal(AuthState::Loading);
    
    // Check initial auth state
    spawn_local(async move {
        if let Some(_user) = get_user() {
            // Verify token is still valid
            match api_get_current_user().await {
                Ok(current_user) => {
                    set_state.set(AuthState::Authenticated(current_user));
                }
                Err(_) => {
                    // Token is invalid, clear storage
                    remove_token();
                    remove_user();
                    set_state.set(AuthState::Unauthenticated);
                }
            }
        } else {
            set_state.set(AuthState::Unauthenticated);
        }
    });

    let context = AuthContext {
        state,
        set_state,
    };

    provide_context(context);
    context
}

// Login function
pub async fn login(username: String, password: String, set_state: WriteSignal<AuthState>) -> Result<(), String> {
    match api_login(username, password).await {
        Ok(response) => {
            if response.success {
                if let (Some(user), Some(token)) = (response.user, response.token) {
                    save_token(&token);
                    save_user(&user);
                    set_state.set(AuthState::Authenticated(user));
                    Ok(())
                } else {
                    Err("Invalid response from server".to_string())
                }
            } else {
                Err(response.message)
            }
        }
        Err(e) => Err(e),
    }
}

// Register function  
pub async fn register(username: String, email: String, password: String, set_state: WriteSignal<AuthState>) -> Result<(), String> {
    match api_register(username, email, password).await {
        Ok(response) => {
            if response.success {
                if let (Some(user), Some(token)) = (response.user, response.token) {
                    save_token(&token);
                    save_user(&user);
                    set_state.set(AuthState::Authenticated(user));
                    Ok(())
                } else {
                    Err("Invalid response from server".to_string())
                }
            } else {
                Err(response.message)
            }
        }
        Err(e) => Err(e),
    }
}

// Logout function
pub fn logout(set_state: WriteSignal<AuthState>) {
    remove_token();
    remove_user();
    set_state.set(AuthState::Unauthenticated);
}
