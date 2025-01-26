use crate::constants::constants::MONGO_URI;

use mongodb::Client;

#[allow(dead_code)]
pub struct DBConf { }

impl DBConf {
    pub async fn connect() -> Client {
        return match Client::with_uri_str(MONGO_URI.as_str()).await {
            Ok(client) => client,
            Err(e) => {
                eprintln!("Failed to connect to MongoDB: {}", e);
                std::process::exit(1);
            }
        };
    }
}
