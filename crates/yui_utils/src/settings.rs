use dotenv::dotenv;
use std::env;

#[derive(Clone)]
pub struct Settings {
    pub mongodb_uri: String,
    pub mongodb_name: String,
}

impl Settings {
    pub fn init() -> Self {
        dotenv().ok();
        let mongodb_uri = match env::var("MONGODB_URI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("[{}] is not present in the environment!", "MONGODB_URI"),
        };
        let mongodb_name = match env::var("MONGODB_NAME") {
            Ok(v) => v.to_string(),
            Err(_) => format!("[{}] is not present in the environment!", "MONGODB_NAME"),
        };
        Self {
            mongodb_uri: mongodb_uri,
            mongodb_name: mongodb_name,
        }
    }
}
