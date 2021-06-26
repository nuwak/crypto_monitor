use std::{thread, time};
use diesel::{RunQueryDsl};
use rest_client::{establish_connection, update_symbol};
use rest_client::models::*;
use crate::binance::Price;
use std::collections::HashMap;

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

        let symbols = symbol.load::<Symbol>(&conn).expect("Can't read symbols").into_iter()
            .map(|it| (it.name, it.id)).collect::<HashMap<String, i64>>();
        dbg!(&symbols);
        let prices: Vec<&Price> = resp.iter().filter(|it| symbols.contains_key(&it.symbol)).collect();
        // dbg!(&prices);
        for price in prices {
            update_symbol(&conn, &price.to_db(), &symbols.get(&price.symbol).unwrap());
        }

        thread::sleep(time::Duration::from_secs(10));
    }
}

