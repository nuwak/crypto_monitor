extern crate diesel;

use self::diesel::prelude::*;
use crypto_monitor::*;
use crypto_monitor::models::Symbol;


fn main() {
    use crypto_monitor::schema::symbol::dsl::*;

    let connection = establish_connection();
    let results = symbol
        // .filter(published.eq(true))
        .limit(5)
        .load::<Symbol>(&connection)
        .expect("Error loading symbol");

    println!("Displaying {} symbols", results.len());
    for post in results {
        println!("{:#?}", post);
        println!("----------\n");
        // println!("{}", post.last_price);
    }
}
// fn main() {
//
// }