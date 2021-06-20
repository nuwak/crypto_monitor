use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;

#[derive(Queryable, Debug)]
pub struct Symbol {
    pub id: i64,
    pub name: String,
    pub last_price: BigDecimal,
    pub low_price: BigDecimal,
    pub high_price: BigDecimal,
    pub change: BigDecimal,
    pub watchlist_id: i64,
    pub volume: BigDecimal,
    pub prev_close_price: BigDecimal,
    pub ask_qty: BigDecimal,
    pub bid_qty: BigDecimal,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

use super::schema::symbol;

#[derive(Insertable, Debug)]
#[table_name="symbol"]
pub struct NewSymbol {
    pub name: String,
    pub last_price: BigDecimal,
    pub low_price: BigDecimal,
    pub high_price: BigDecimal,
    pub change: BigDecimal,
    pub watchlist_id: i64,
    pub volume: BigDecimal,
    pub prev_close_price: BigDecimal,
    pub ask_qty: BigDecimal,
    pub bid_qty: BigDecimal,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}