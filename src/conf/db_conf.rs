use crate::constants::constants::MONGO_URI;
use lazy_static::lazy_static;
use mongodb::{bson::Document, Client, Collection, Database};
use std::sync::Mutex;

lazy_static! {
    static ref DB_CLIENT: Mutex<Option<Client>> = Mutex::new(None);
    static ref DB: Mutex<Option<Database>> = Mutex::new(None);
}

pub struct DBCollections {
    // pub users: Collection<Document>,
    pub movies: Collection<Document>,
}

impl DBCollections {
    pub fn new(db: &Database) -> Self {
        Self {
            // users: db.collection("users"),
            movies: db.collection("movies"),
        }
    }
}

pub struct DBConf {}

impl DBConf {
    pub async fn init() -> DBCollections {
        let client = match Client::with_uri_str(MONGO_URI.as_str()).await {
            Ok(client) => client,
            Err(e) => {
                eprintln!("Failed to connect to MongoDB: {}", e);
                std::process::exit(1);
            }
        };

        let db = client.database("sample_mflix");
        let collections = DBCollections::new(&db);

        *DB_CLIENT.lock().unwrap() = Some(client);
        *DB.lock().unwrap() = Some(db);

        collections
    }

    // pub fn get_collections() -> Option<DBCollections> {
    //     DB.lock().unwrap().as_ref().map(DBCollections::new)
    // }
}
