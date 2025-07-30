use crate::entity::user::User;
use crate::entity::role_type::Role;
use crate::util::neon_client::NeonClient;
use worker::console_log;

/// User repository with Neon database integration
#[derive(Clone)]
pub struct UserRepository {
    neon_client: NeonClient,
}

impl UserRepository {
    pub fn new(connection_string: String) -> Self {
        let neon_client = NeonClient::new(
            "ep-wispy-bread-ae0fl1we".to_string(), // Use the correct preferred project ID
            "neondb".to_string(),
            connection_string,
        );
        Self { neon_client }
    }

    pub async fn get_user(&self, username: &str) -> Result<Option<User>, String> {
        console_log!("LIVE DATABASE: Looking up user in Neon database: {}", username);

        // Use the Neon client to query the live database
        match self.neon_client.find_user_by_username(username).await {
            Ok(Some(user_data)) => {
                console_log!("LIVE DATABASE: Found user data in Neon database: {:?}", user_data);

                // Convert the JSON user data back to User entity
                if let (Some(id), Some(username), Some(email), Some(password_hash)) = (
                    user_data.get("id").and_then(|v| v.as_str()),
                    user_data.get("username").and_then(|v| v.as_str()),
                    user_data.get("email").and_then(|v| v.as_str()),
                    user_data.get("password_hash").and_then(|v| v.as_str()),
                ) {
                    // Create User directly with the hash from database (don't use User::new which adds "hashed_" prefix)
                    let user = User {
                        id: id.to_string(),
                        email: email.to_string(),
                        username: username.to_string(),
                        password_hash: password_hash.to_string(), // Use the hash directly from database
                        role: Role::User,
                        is_active: true,
                        is_verified: false,
                    };
                    console_log!("LIVE DATABASE: Successfully converted user data for: {}", username);
                    Ok(Some(user))
                } else {
                    console_log!("LIVE DATABASE: Invalid user data format from database");
                    Err("Invalid user data format".to_string())
                }
            }
            Ok(None) => {
                console_log!("LIVE DATABASE: User {} not found in Neon database", username);
                Ok(None)
            }
            Err(e) => {
                console_log!("LIVE DATABASE: Error querying Neon database: {}", e);
                Err(e)
            }
        }
    }

    pub async fn add_user(&self, user: User) -> Result<(), String> {
        console_log!("LIVE DATABASE: Adding user to Neon database: {} with ID: {}", user.username, user.id);

        // Use the Neon client to insert the user into the live database
        match self.neon_client.insert_user(&user.id, &user.username, &user.email, &user.password_hash).await {
            Ok(()) => {
                console_log!("LIVE DATABASE: User {} successfully added to Neon database", user.username);
                Ok(())
            }
            Err(e) => {
                console_log!("LIVE DATABASE: Failed to add user {} to Neon database: {}", user.username, e);
                Err(e)
            }
        }
    }
}