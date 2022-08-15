use serde::{Deserialize, Serialize};
use uuid::Uuid;

use misato_security::{generate_token, password::*};
use misato_utils::get_current_timestamp;

#[derive(Eq, Hash, PartialEq, Debug, Serialize, Deserialize, Default, Clone)]
pub struct UserLog {
    pub ip: String,
    pub timestamp: u64,
}

#[derive(Eq, Hash, PartialEq, Debug, Serialize, Deserialize, Default, Clone)]
pub struct UserToken {
    pub token: String,
    pub timestamp: u64,
    pub expiration_timestamp: u64,
}

#[derive(Eq, Hash, PartialEq, Debug, Serialize, Deserialize, Default, Clone)]
pub struct User {
    pub timestamp: u64,
    pub uuid: String,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<Vec<UserLog>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<Password>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens: Option<Vec<UserToken>>,
    pub access: UserAccess,
}

impl User {
    pub fn create(username: String, password: Password, uuid: Option<Uuid>) -> Self {
        Self {
            timestamp: get_current_timestamp(),
            uuid: {
                match uuid {
                    None => Uuid::new_v4().to_string(),
                    Some(uuid) => uuid.to_string(),
                }
            },
            username,
            password: Some(password),
            ..Default::default()
        }
    }

    pub fn new_token(&mut self, seconds: u64) -> UserToken {
        let token = UserToken {
            token: generate_token(128),
            timestamp: get_current_timestamp(),
            expiration_timestamp: get_current_timestamp() + (seconds * 1000),
        };
        let mut tokens: Vec<UserToken> = if self.tokens.is_some() {
            self.tokens.as_ref().unwrap().to_vec()
        } else {
            Vec::<UserToken>::new()
        };
        tokens.push(token.clone());
        self.tokens = Some(tokens);
        token
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Serialize, Deserialize, Default, Clone)]
pub struct UserAccess {
    pub role: UserRoleType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<UserPermissionType>>,
}

#[derive(Eq, Hash, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum UserRoleType {
    Admin, // Only the main website has access
    User,  // New account
}

impl Default for UserRoleType {
    fn default() -> Self {
        UserRoleType::User
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum UserPermissionType {
    UserManager, // Create, Delete, Edit user informations
    None,        // Default, no more access
}

impl Default for UserPermissionType {
    fn default() -> Self {
        UserPermissionType::None
    }
}
