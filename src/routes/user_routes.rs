use rocket::serde::json::Json;
use crate::models::user_model::User;

use std::env;

#[get("/user/<id>")]
pub fn get_user(id: u32) -> Json<User> {
    let key = env::var("MONGO_DB_STRING_CONNECTION").expect("KEY must be set");
    println!("Using API key: {}", key);
    let user = User {
        id,
        name: "John Doeeee".to_string(),
    };
    Json(user)
}