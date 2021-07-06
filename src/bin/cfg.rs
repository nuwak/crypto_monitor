#[macro_use]
extern crate lazy_static;
extern crate crypto_monitor;
// use crate::config::Config;
use crypto_monitor::config::Config;



use std::error::Error;
use std::sync::RwLock;

lazy_static! {
	static ref SETTINGS: RwLock<Config> = RwLock::new(Config::init());
}

fn main() {
	dbg!(&SETTINGS.read().expect("Can't read settings").db);
	dbg!(&SETTINGS.read().unwrap().db);
}