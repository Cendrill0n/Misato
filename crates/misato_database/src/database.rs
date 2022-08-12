use mongodb::sync::{Client, Collection};

use crate::models::data_model::Data;
use crate::user_manager::*;
use misato_utils::settings::Settings;

pub struct Database {
    pub data: Collection<Data>,
    pub usermanager: UserManager,
}

impl Database {
    pub fn init(settings: &Settings) -> Self {
        let uri = &settings.mongodb_uri;
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database(&settings.mongodb_name);
        Database {
            data: db.collection("data"),
            usermanager: UserManager::init(db.collection("users")),
        }
    }
}
