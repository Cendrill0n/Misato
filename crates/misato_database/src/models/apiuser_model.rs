use serde::{Deserialize, Serialize};

use misato_security::generate_token;
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<ApiUserToken>,
    pub access: ApiUserAccess,
}

impl ApiUser {
    pub fn create_default(token: String) -> Self {
        Self {
            timestamp: 0,
            uuid: "admin".to_string(),
            token: Some(ApiUserToken {
                token,
                timestamp: get_current_timestamp(),
                expiration_timestamp: i64::MAX as u64,
            }),
            access: ApiUserAccess {
                role: ApiUserRoleType::Admin,
                permissions: None,
            },
        }
    }
    pub fn create(uuid: String) -> Self {
        Self {
            timestamp: get_current_timestamp(),
            uuid,
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
