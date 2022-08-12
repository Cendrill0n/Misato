use serde::{Deserialize, Serialize};
use uuid::Uuid;

use misato_security::password::*;

#[derive(Eq, Hash, PartialEq, Debug, Serialize, Deserialize, Default, Clone)]
pub struct UserLog {
    pub ip: String,
    pub timestamp: u64,
}

#[derive(Eq, Hash, PartialEq, Debug, Serialize, Deserialize, Default, Clone)]
pub struct User {
    pub uuid: Uuid,
    pub username: String,
    pub logs: Vec<UserLog>,
    pub password: Password,
}

impl User {
    pub fn create(username: String, password: Password, uuid: Option<Uuid>) -> Self {
        User {
            uuid: {
                match uuid {
                    None => Uuid::new_v4(),
                    Some(uuid) => uuid,
                }
            },
            username,
            password,
            ..Default::default()
        }
    }
}
