extern crate dotenv;
use std::{thread, time};
use diesel::{RunQueryDsl};
use crypto_monitor::{establish_connection, update_symbol};
use crypto_monitor::models::*;
use crate::binance::Price;
use crate::config::Config;
use std::collections::HashMap;
use teloxide::prelude::*;
use teloxide::types::ParseMode;
use std::env;
use chrono::{Local, Utc, Timelike};
use bigdecimal::BigDecimal;


mod binance;
mod config;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use crypto_monitor::schema::symbol::dsl::*;

    let config = Config::init();

    // let chat_id = env::var(&"TELEGRAM_CHAT_ID").unwrap().parse::<i64>().unwrap();

    let time = "%Y-%m-%d %H:%M:%S";
    let conn = establish_connection(&config.db);
    teloxide::enable_logging!();
    log::info!("Starting dices_bot...");

    let bot = Bot::new(&config.telegram.token).auto_send();

    loop {
        let hour = Utc::now().hour();
        if hour > 20 || hour < 6 {
            log::info!("[{}] {}", Local::now().format(&time), "continue");
            continue;
        }

        let resp = match reqwest::get(&config.api).await {
            Ok(resp) => {
                match resp.json::<Vec<Price>>().await {
                    Ok(resp) => resp,
                    _ => {
                        log::error!("[{}] {}", Local::now().format(&time), "response parsing error");
                        thread::sleep(time::Duration::from_secs(60));
                        continue;
                    }
                }
            }
            _ => {
                log::error!("[{}] {}", Local::now().format(&time), "request error");
                thread::sleep(time::Duration::from_secs(60));
                continue;
            }
        };

        let symbols: Vec<Symbol> = symbol.load::<Symbol>(&conn).expect("Can't read symbols").into_iter().collect();
        let mut symbol_map: HashMap<&String, &Symbol> = HashMap::new();
        for sym in &symbols {
            symbol_map.insert(&sym.name, sym);
        }
        let prices: Vec<&Price> = resp.iter().filter(|&it| symbol_map.contains_key(&it.symbol)).collect();
        for price in prices {
            let price_new = price.to_db();
            let row = symbol_map.get(&price.symbol).unwrap();
            let price_change = &price_new.change / &price_new.prev_close_price * BigDecimal::from(100);

            if price_new.high_price > row.high_price {
                let message = format!(
                    "`New HIGH price: {} {:10.2} {:10.2}`",
                    row.name,
                    price_new.high_price,
                    price_change,
                );
                log::info!("[{}] {}", Local::now().format(&time), &message);
                bot.send_message(config.telegram.chat_id, message)
                    .parse_mode(ParseMode::MarkdownV2)
                    .send();
            } else if price_new.low_price < row.low_price {
                let message = format!(
                    "`New LOW price: {} {:10.2} {:10.2}`",
                    row.name,
                    price_new.low_price,
                    price_change,
                );
                log::info!("[{}] {}", Local::now().format(&time), &message);
                bot.send_message(config.telegram.chat_id, message)
                    .parse_mode(ParseMode::MarkdownV2)
                    .send();
            }

            update_symbol(&conn, &price_new, &row.id);
        }

        log::info!("[{}] finished iteration", Local::now().format(&time));
        thread::sleep(time::Duration::from_secs(60));
    }
}

