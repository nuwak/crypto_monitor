use std::io::Write;
use env_logger::Builder;
use log::LevelFilter;
use chrono::{Local};
use crate::config::Config;
use crypto_monitor::establish_connection;
use diesel::PgConnection;
use teloxide::prelude::*;

pub fn log_init()
{
    Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                     "{} [{}] - {}",
                     Local::now().format("%Y-%m-%d %H:%M:%S"),
                     record.level(),
                     record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();
}

pub fn boot() -> (Config, PgConnection, AutoSend<Bot>)
{
    let config = Config::init();
    dbg!(&config);
    let conn = establish_connection(&config.db);
    let bot = Bot::new(&config.telegram.token).auto_send();
    return (config, conn, bot);
}