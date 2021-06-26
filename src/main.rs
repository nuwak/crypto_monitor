use std::{thread, time};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use rest_client::{create_symbol, establish_connection, update_symbol};
use rest_client::models::*;
use crate::binance::Price;

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
            .map(|it| it.name).collect::<Vec<String>>();
        dbg!(&symbols);
        let prices: Vec<&Price> = resp.iter().filter(|it| symbols.contains(&it.symbol)).collect();
        dbg!(&prices);
        let new_symbol = prices[0].to_db();
        for price in prices {
            update_symbol(&conn, &price.to_db(), &price.id);
        }
        match symbol.filter(name.eq("BTCUSDT")).first::<Symbol>(&conn) {
            Ok(res) => {
                let count = update_symbol(&conn, &new_symbol, &res.id);
                println!("updated rows {:#?}", count);
                println!("{:#?}", res)
            }
            Err(_) => {
                create_symbol(&conn, &new_symbol);
                println!("save symbol")
            }
        };

        thread::sleep(time::Duration::from_secs(10));
    }

    // Ok(())
}

