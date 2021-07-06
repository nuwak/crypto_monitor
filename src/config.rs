extern crate dotenv;
use dotenv::dotenv;
use std::env;

#[derive(Debug)]
pub struct Telegram {
    pub token: String,
    pub chat_id: i64,
}

#[derive(Debug)]
pub struct Config {
    pub db: String,
    pub api: String,
    pub telegram: Telegram,
}

impl Config {
    pub fn init() -> Self {
        dotenv().expect(".env loading fail");
        // dotenv().ok();

        Config {
            db: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            api: String::from("https://api.binance.com/api/v3/ticker/24hr"),
            telegram: Telegram {
                token: env::var("TELEGRAM_TOKEN").expect("TELEGRAM_TOKEN must be set"),
                chat_id: env::var(&"TELEGRAM_CHAT_ID").expect("TELEGRAM_CHAT_ID must be set").parse().expect("TELEGRAM_CHAT_ID not int"),
            },
        }
    }
}