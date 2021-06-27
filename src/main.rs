use std::{thread, time};
use diesel::{RunQueryDsl};
use rest_client::{establish_connection, update_symbol};
use rest_client::models::*;
use crate::binance::Price;
use std::collections::HashMap;
use teloxide::prelude::*;
use teloxide::types::ParseMode;
use std::env;
use chrono::Local;


mod binance;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use rest_client::schema::symbol::dsl::*;
    let time = "%Y-%m-%d %H:%M:%S";

    let conn = establish_connection();
    teloxide::enable_logging!();
    log::info!("Starting dices_bot...");
    let token = env::var("TELEGRAM_TOKEN").expect("TELEGRAM_TOKEN must be set");
    let chat_id: i64 = env::var("TELEGRAM_CHAT_ID").unwrap().parse().unwrap();
    dbg!(&chat_id);
    let bot = Bot::new(token).auto_send();

    loop {
        let resp = reqwest::get("https://api.binance.com/api/v3/ticker/24hr")
            .await?
            .json::<Vec<Price>>()
            .await?;

        let symbols: Vec<Symbol> = symbol.load::<Symbol>(&conn).expect("Can't read symbols").into_iter().collect();
        let mut symbol_map: HashMap<&String, &Symbol> = HashMap::new();
        for sym in &symbols {
            symbol_map.insert(&sym.name, sym);
        }
        let prices: Vec<&Price> = resp.iter().filter(|&it| symbol_map.contains_key(&it.symbol)).collect();
        for price in prices {
            let price_new = price.to_db();
            let row = symbol_map.get(&price.symbol).unwrap();

            if price_new.high_price > row.high_price {
                let message = format!(
                    "`New HIGH price: {} {:10.2} {:10.2}`",
                    row.name,
                    price_new.high_price,
                    price_new.last_price,
                );
                log::info!("[{}] {}", Local::now().format(&time), &message);
                bot.send_message(chat_id, message)
                    .parse_mode(ParseMode::MarkdownV2)
                    .send().await;
            } else if price_new.low_price < row.low_price {
                let message = format!(
                    "`New LOW price: {} {:10.2} {:10.2}`",
                    row.name,
                    price_new.high_price,
                    price_new.last_price,
                );
                log::info!("[{}] {}", Local::now().format(&time), &message);
                bot.send_message(chat_id, message)
                    .parse_mode(ParseMode::MarkdownV2)
                    .send().await;
            }

            update_symbol(&conn, &price_new, &row.id);
        }

        log::info!("[{}] finished iteration", Local::now().format(&time));
        thread::sleep(time::Duration::from_secs(60));
    }
}

