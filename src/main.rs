#[macro_use]
extern crate rocket;

use dotenv::dotenv;

mod constants;
mod models;
mod routes;

use routes::user_routes::*;

use crate::constants::constants::MONGO_URI;

#[get("/")]
fn index() -> &'static str {
    println!("Lazyyyyy keyyyyy: {}", MONGO_URI.to_string());
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

    // start rocket server and configure routes
    let _rocket = rocket::build()
        .mount("/", routes![index])
        .mount("/api/v1", routes![get_user])
        .launch()
        .await?;

    Ok(())
}
