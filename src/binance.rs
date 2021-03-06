use bigdecimal::BigDecimal;
use std::str::FromStr;
use chrono::{Utc};
use crypto_monitor::models::NewSymbol;
// use log::{error};
// use reqwest::Error;

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    pub symbol: String,
    pub price_change: String,
    pub price_change_percent: String,
    pub weighted_avg_price: String,
    pub prev_close_price: String,
    pub last_price: String,
    pub last_qty: String,
    pub bid_price: String,
    pub bid_qty: String,
    pub ask_price: String,
    pub ask_qty: String,
    pub open_price: String,
    pub high_price: String,
    pub low_price: String,
    pub volume: String,
    pub quote_volume: String,
    pub last_id: i64,
    pub count: i64,
}

impl Price {
    pub fn to_db(&self) -> NewSymbol {
        NewSymbol {
            name: &self.symbol,
            last_price: BigDecimal::from_str(self.last_price.as_str()).unwrap(),
            low_price: BigDecimal::from_str(self.low_price.as_str()).unwrap(),
            high_price: BigDecimal::from_str(self.high_price.as_str()).unwrap(),
            change: BigDecimal::from_str(self.price_change.as_str()).unwrap(),
            watchlist_id: 0,
            volume: BigDecimal::from_str(self.volume.as_str()).unwrap(),
            prev_close_price: BigDecimal::from_str(self.prev_close_price.as_str()).unwrap(),
            ask_qty: BigDecimal::from_str(self.ask_qty.as_str()).unwrap(),
            bid_qty: BigDecimal::from_str(self.bid_qty.as_str()).unwrap(),
            updated_at: Utc::now().naive_utc(),
            created_at: Utc::now().naive_utc()
        }
    }

    // pub async fn get() -> Result<Vec<Price>, Error>
    // {
    //     return match reqwest::get(&config.api).await {
    //         Ok(resp) => {
    //             match resp.json::<Vec<Price>>().await {
    //                 Ok(resp) => Ok(resp),
    //                 Error => {
    //                     error!("response parsing error");
    //                     Error
    //                 }
    //             }
    //         }
    //         _ => {
    //             error!("request error");
    //         }
    //     };
    // }
}
