use mongodb::{error::Error, *};

use crate::api_manager::*;
use crate::models::data_model::Data;
use crate::user_manager::*;
use misato_utils::settings::Settings;

pub struct Database {
    pub data: Collection<Data>,
    pub usermanager: UserManager,
    pub apiusermanager: ApiUserManager,
}

impl Database {
    pub async fn init(settings: &Settings) -> Result<Self, Error> {
        let uri = &settings.mongodb_uri;
        let client = Client::with_uri_str(uri).await?;
        let db = client.database(&settings.mongodb_name);
        Ok(Database {
            data: db.collection("data"),
            usermanager: UserManager::init(db.collection("users")),
            apiusermanager: ApiUserManager::init(db.collection("apiusers")),
        })
    }
}
