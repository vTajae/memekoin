use crate::entity::role_type::Role;
use crate::entity::user::User;
use crate::dto::user::{RegisterCommand, LoginCommand, UserDTO, LoginResponse};
use crate::service::auth::AuthenticationService;
use crate::repo::user::UserRepository;
use crate::state::AppState;
use uuid::Uuid;
use worker::*;

#[derive(Debug)]
pub enum UserErrors {
    Exists,
    InvalidPassword,
    UnknownFailure,
}

impl RegisterCommand {
    pub async fn handle(
        &self,
        user_repository: &UserRepository,
    ) -> std::result::Result<UserDTO, UserErrors> {
        let role = Role::User; // Default role for new users
        
        let user = User::new(
            Uuid::new_v4(),
            self.email.clone(),
            self.username.clone(),
            self.password.clone(),
            role,
            true,
            false,
        );

        console_log!(
            "Attempting to create id: {} user: {} with role: {:?}",
            &user.id,
            &user.username,
            &user.role
        );

        match user_repository.get_user(&user.username).await {
            Ok(existing_user) => match existing_user {
                Some(_) => {
                    console_log!("User already exists");
                    Err(UserErrors::Exists)
                }
                None => match user_repository.add_user(user).await {
                    Ok(_) => Ok(UserDTO {
                        username: self.username.clone(),
                        email: self.email.clone(),
                    }),
                    Err(e) => {
                        console_log!("Error creating user: {}", e);
                        Err(UserErrors::UnknownFailure)
                    }
                },
            },
            Err(_) => Err(UserErrors::UnknownFailure),
        }
    }
}

pub async fn handle_register(mut req: Request, ctx: RouteContext<AppState>) -> Result<Response> {
    console_log!("LIVE DATABASE: Handling register request");

    let command: RegisterCommand = match req.json::<RegisterCommand>().await {
        Ok(cmd) => {
            console_log!("LIVE DATABASE: Successfully parsed register request for user: {}", cmd.username);
            cmd
        }
        Err(e) => {
            console_log!("LIVE DATABASE: Failed to parse JSON request: {}", e);
            return Response::error("Invalid JSON request", 400);
        }
    };

    match command.handle(&ctx.data.user_repository).await {
        Ok(user_result) => {
            console_log!("LIVE DATABASE: Registration successful for user: {}", user_result.username);
            Response::from_json(&user_result)
        }
        Err(e) => {
            console_log!("LIVE DATABASE: Registration failed with error: {:?}", e);
            match e {
                UserErrors::UnknownFailure => Response::error("Failed to register user", 500),
                UserErrors::Exists => Response::error("User exists", 400),
                UserErrors::InvalidPassword => Response::error("Failed to register user", 500),
            }
        }
    }
}

impl LoginCommand {
    pub async fn handle(
        &self,
        user_repository: &UserRepository,
        auth_service: &AuthenticationService,
    ) -> std::result::Result<LoginResponse, UserErrors> {
        if let Ok(user) = user_repository.get_user(&self.username).await {
            return match user {
                Some(user) => {
                    if user.verify_password(&self.password) {
                        return match auth_service.generate_token_for(self.username.clone()) {
                            Ok(token) => Ok(LoginResponse { token }),
                            Err(_) => Err(UserErrors::UnknownFailure),
                        };
                    }
                    Err(UserErrors::InvalidPassword)
                }
                None => Err(UserErrors::UnknownFailure),
            };
        }
        Err(UserErrors::InvalidPassword)
    }
}

pub async fn handle_login(mut req: Request, ctx: RouteContext<AppState>) -> Result<Response> {
    console_log!("LIVE DATABASE: Handling login request");

    let command: LoginCommand = match req.json::<LoginCommand>().await {
        Ok(cmd) => {
            console_log!("LIVE DATABASE: Successfully parsed login request for user: {}", cmd.username);
            cmd
        }
        Err(e) => {
            console_log!("LIVE DATABASE: Failed to parse JSON request: {}", e);
            return Response::error("Invalid JSON request", 400);
        }
    };

    match command
        .handle(&ctx.data.user_repository, &ctx.data.auth_service)
        .await
    {
        Ok(resp) => {
            console_log!("LIVE DATABASE: Login successful, returning token");
            Response::from_json(&resp)
        }
        Err(e) => {
            console_log!("LIVE DATABASE: Login failed with error: {:?}", e);
            match e {
                UserErrors::InvalidPassword => Response::error("Unauthorized", 401),
                UserErrors::Exists => Response::error("User exists", 400),
                UserErrors::UnknownFailure => Response::error("Failed", 500),
            }
        }
    }
}
