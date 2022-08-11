extern crate misato_utils;
use misato_utils::settings::Settings;

use crate::models::data_model::Data;

use mongodb::sync::{Client, Collection};

pub struct Database {
    pub data: Collection<Data>,
}

impl Clone for Database {
    fn clone(&self) -> Self {
        Database {
            data: self.data.clone_with_type(),
        }
    }
}

impl Database {
    pub fn init(settings: &Settings) -> Self {
        let uri = &settings.mongodb_uri;
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database(&settings.mongodb_name);
        Database {
            data: db.collection("data"),
        }
    }
}
