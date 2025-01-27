use crate::{conf::db_conf::DBCollections, models::user_model::User};
use mongodb::bson::doc;
use rocket::{http::Status, serde::json::Json, State};

#[get("/user/<id>")]
pub async fn get_user(id: u32, collections: &State<DBCollections>) -> Result<Json<User>, Status> {
    let movie = match collections
        .movies
        .find_one(doc! { "title": "The Perils of Pauline" })
        .await
    {
        Ok(movie) => movie,
        Err(e) => {
            eprintln!("Failed to find movie: {}", e);
            std::process::exit(1);
        }
    };

    println!("Found a movie:\n{:#?}", movie);

    let user = User {
        id,
        name: "John Doeeee".to_string(),
    };

    Ok(Json(user))
}
