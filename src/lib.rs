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

use crate::schema::symbol::columns::id;
use crate::schema::symbol;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    println!("{:#?}", database_url);
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_symbol<'a>(conn: &PgConnection, new_symbol: &'a NewSymbol) -> Symbol {

    diesel::insert_into(symbol::table)
        .values(&*new_symbol)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn update_symbol<'a>(conn: &PgConnection, new_symbol: &'a NewSymbol, symbol_id: &'a i64) -> usize {
    // use crate::schema::symbol;
    // use diesel::prelude::*;

    diesel::update(symbol::table)
        .filter(id.eq(symbol_id))
        .set(&*new_symbol)
        .execute(conn)
        .expect("Error saving new symbol")
}


