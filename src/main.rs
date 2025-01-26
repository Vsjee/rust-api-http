#[macro_use]
extern crate rocket;

use mongodb::{
    bson::{doc, Document},
    Collection,
};

use dotenv::dotenv;

mod conf;
mod constants;
mod models;
mod routes;

use routes::user_routes::*;

use crate::conf::db_conf::DBConf;

#[get("/")]
fn index() -> &'static str {
    "Hello, Rocket API!"
}

// #[launch]
// fn rocket() -> _ {
//     rocket::build()
//         .mount("/", routes![index])
//         .mount("/api/v1", routes![get_user])
// }

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // load .env file
    dotenv().ok();

    // connect to MongoDB
    let client = DBConf::connect().await;

    let db: mongodb::Database = client.database("rust-api-http");
    println!("Connected to MongoDB: {:?}", db);

    println!("-----------");
    let database = client.database("sample_mflix");
    let my_coll: Collection<Document> = database.collection("movies");
    let my_movie = match my_coll
        .find_one(doc! { "title": "The Perils of Pauline" })
        .await
    {
        Ok(movie) => movie,
        Err(e) => {
            eprintln!("Failed to find movie: {}", e);
            std::process::exit(1);
        }
    };
    println!("Found a movie:\n{:#?}", my_movie);

    // start rocket server and configure routes
    let _rocket = rocket::build()
        .mount("/", routes![index])
        .mount("/api/v1", routes![get_user])
        .launch()
        .await?;

    Ok(())
}
