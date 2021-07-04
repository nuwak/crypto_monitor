pub mod schema;
pub mod models;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use crate::models::{NewSymbol, Symbol};

use crate::schema::symbol::columns::id;
use crate::schema::symbol;


pub fn establish_connection(database_url: &String) -> PgConnection {
    PgConnection::establish(database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_symbol<'a>(conn: &PgConnection, new_symbol: &'a NewSymbol) -> Symbol {
    diesel::insert_into(symbol::table)
        .values(&*new_symbol)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn update_symbol<'a>(conn: &PgConnection, new_symbol: &'a NewSymbol, symbol_id: &'a i64) -> usize {
    diesel::update(symbol::table)
        .filter(id.eq(symbol_id))
        .set(&*new_symbol)
        .execute(conn)
        .expect("Error saving new symbol")
}


