use crate::models::user_model::User;
use rocket::serde::json::Json;

#[get("/user/<id>")]
pub fn get_user(id: u32) -> Json<User> {
    let user = User {
        id,
        name: "John Doeeee".to_string(),
    };
    Json(user)
}
