use std::{thread, time};
use std::str::FromStr;

use bigdecimal::BigDecimal;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use diesel::associations::HasTable;
use diesel::prelude::*;

use rest_client::{establish_connection, create_symbol};
use rest_client::models::*;
use rest_client::schema::symbol;

use crate::binance::{Price};

mod binance;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = establish_connection();
    use rest_client::schema::symbol::dsl::*;

    loop {
        let resp = reqwest::get("https://api.binance.com/api/v3/ticker/24hr")
            .await?
            .json::<Vec<Price>>()
            .await?;

        let res: Vec<&Price> = resp.iter().filter(|it| it.symbol == "BTCUSDT").collect();

        println!("{:#?}", res);
        let new_symbol = res[0].to_db();
        match symbol.filter(name.eq("BTCUSDT")).first::<Symbol>(&conn) {
            Ok(res) => {
                // let res_symbol = diesel::update(symbol)
                //     .filter(id.eq(res.id))
                //     .set(&new_symbol)
                //     .execute(&conn)
                //     .expect("Error saving new symbol");
                //
                // println!("update {:#?}", res_symbol);
                println!("{:#?}", res)
            },
            Err(NotFound) => {
                // let res_symbol: Symbol = diesel::insert_into(symbol::table)
                //     .values(&new_symbol)
                //     .get_result(&conn)
                //     .expect("Error saving new symbol");

                create_symbol(&conn, &new_symbol);
                // let symbol_result = diesel::insert_into(symbol::table)
                //     .values(&new_symbol)
                //     .get_result(&conn)
                //     .expect("Error saving new post");
                println!("save symbol")

                // println!("save {:#?}", res_symbol);
            }
        };

        thread::sleep(time::Duration::from_secs(10));
    }

    // Ok(())
}

