extern crate dotenv;
use dotenv::dotenv;
use std::env;

fn main() {
    let db= env::var("DATABASE_URL").expect("TELEGRAM_TOKEN must be set");
    dbg!(&db);
    // token: env::var("TELEGRAM_TOKEN").expect("TELEGRAM_TOKEN must be set"),
    // chat_id: env::var("TELEGRAM_CHAT_ID").unwrap().parse().ok(),
}

// extern crate dotenv;
//
// use dotenv::dotenv;
// use std::env;
//
// fn main() {
//     dotenv().ok();
//
//     for (key, value) in env::vars() {
//         println!("{}: {}", key, value);
//     }
//     let db = env::var("DATABASE_URL");
//     dbg!(db);
// }