table! {
    symbol (id) {
        id -> Int8,
        name -> Varchar,
        last_price -> Numeric,
        low_price -> Numeric,
        high_price -> Numeric,
        change -> Numeric,
        watchlist_id -> Int8,
        volume -> Numeric,
        prev_close_price -> Numeric,
        ask_qty -> Numeric,
        bid_qty -> Numeric,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}
