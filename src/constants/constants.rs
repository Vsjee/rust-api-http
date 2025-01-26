use lazy_static::lazy_static;
use std::env;

// lazy_static! {
//     pub static ref MONGO_URI: String = {
//         env::var("MONGO_DB_STRING_CONNECTION").expect("KEY must be set")
//     }
// }

lazy_static! {
    pub static ref MONGO_URI: String =
        env::var("MONGO_DB_STRING_CONNECTION").expect("MONGO_DB_STRING_CONNECTION must be set");
}
