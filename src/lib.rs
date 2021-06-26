pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use crate::models::{NewSymbol, Symbol};


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    println!("{:#?}", database_url);
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_symbol<'a>(conn: &PgConnection, new_symbol: &'a NewSymbol) -> Symbol {
    use crate::schema::symbol;

    diesel::insert_into(symbol::table)
        .values(&*new_symbol)
        .get_result(conn)
        .expect("Error saving new post")
}
