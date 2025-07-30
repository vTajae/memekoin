use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct UserDTO {
    pub username: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct RegisterCommand {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginCommand {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}
