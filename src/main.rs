extern crate dotenv;

use std::{thread, time};
use diesel::{RunQueryDsl};
use crypto_monitor::{update_symbol};
use crypto_monitor::models::*;
use crate::binance::Price;
use std::collections::HashMap;
use teloxide::prelude::*;
use teloxide::types::ParseMode;
use chrono::{Utc, Timelike};
use bigdecimal::BigDecimal;
use crate::bootstrap::{log_init, boot};
use log::{error, info};


mod binance;
mod bootstrap;
mod config;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use crypto_monitor::schema::symbol::dsl::*;
    log_init();
    let (config, conn, bot) = boot();
    bot.send_message(config.telegram.chat_id, "start").send().await;

    info!("Starting dices_bot...");

    loop {
        let hour = Utc::now().hour();
        if hour > 20 || hour < 6 {
            info!("continue");
            thread::sleep(time::Duration::from_secs(60));
            continue;
        }

        let resp = match reqwest::get(&config.api).await {
            Ok(resp) => {
                match resp.json::<Vec<Price>>().await {
                    Ok(resp) => resp,
                    _ => {
                        error!("response parsing error");
                        thread::sleep(time::Duration::from_secs(60));
                        continue;
                    }
                }
            }
            _ => {
                error!("request error");
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
                info!("{}", &message);
                bot.send_message(config.telegram.chat_id, message)
                    .parse_mode(ParseMode::MarkdownV2)
                    .send().await;
            } else if price_new.low_price < row.low_price {
                let message = format!(
                    "`New LOW price: {} {:10.2} {:10.2}`",
                    row.name,
                    price_new.low_price,
                    price_change,
                );
                info!("{}", &message);
                bot.send_message(config.telegram.chat_id, message)
                    .parse_mode(ParseMode::MarkdownV2)
                    .send().await;
            }

            update_symbol(&conn, &price_new, &row.id);
        }

        info!("finished iteration");
        thread::sleep(time::Duration::from_secs(60));
    }
}

