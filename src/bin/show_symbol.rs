extern crate diesel;

use self::diesel::prelude::*;
use rest_client::*;
use rest_client::models::Symbol;


fn main() {
    use rest_client::schema::symbol::dsl::*;

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