use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    Admin,
    User,
    System,
}

impl Role {
    pub fn from_i32(value: i32) -> Option<Role> {
        match value {
            1 => Some(Role::Admin),
            2 => Some(Role::User),
            3 => Some(Role::System),
            _ => None,
        }
    }

    pub fn to_i32(&self) -> i32 {
        match self {
            Role::Admin => 1,
            Role::User => 2,
            Role::System => 3,
        }
    }
}
