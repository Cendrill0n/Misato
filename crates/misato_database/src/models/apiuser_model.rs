use serde::{Deserialize, Serialize};
use uuid::Uuid;

use misato_security::{generate_token, password::*};
use misato_utils::get_current_timestamp;

#[derive(Eq, Hash, PartialEq, Debug, Serialize, Deserialize, Default, Clone)]
pub struct ApiUserToken {
    pub token: String,
    pub timestamp: u64,
    pub expiration_timestamp: u64,
}

#[derive(Eq, Hash, PartialEq, Debug, Serialize, Deserialize, Default, Clone)]
pub struct ApiUser {
    pub timestamp: u64,
    pub uuid: String,
    pub username: String,
    pub password: Password,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<ApiUserToken>,
    pub access: ApiUserAccess,
}

impl ApiUser {
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
            password,
            ..Default::default()
        }
    }

    pub fn new_token(&mut self, seconds: u64) -> ApiUserToken {
        let token = ApiUserToken {
            token: generate_token(128),
            timestamp: get_current_timestamp(),
            expiration_timestamp: get_current_timestamp() + (seconds * 1000),
        };
        self.token = Some(token.clone());
        token
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Serialize, Deserialize, Default, Clone)]
pub struct ApiUserAccess {
    pub role: ApiUserRoleType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ApiUserPermissionType>>,
}

#[derive(Eq, Hash, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum ApiUserRoleType {
    Admin, // Only the main website has access
    Dev,   // Verified USER
    User,  // New account
}

impl Default for ApiUserRoleType {
    fn default() -> Self {
        ApiUserRoleType::User
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum ApiUserPermissionType {
    UserManager, // Create, Delete, Edit user informations
    None,        // Default, no more access
}

impl Default for ApiUserPermissionType {
    fn default() -> Self {
        ApiUserPermissionType::None
    }
}
