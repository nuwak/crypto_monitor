use std::collections::HashMap;

fn main() {
    let tuples = vec![("one", 1), ("two", 2), ("three", 3)];
    let m: HashMap<_, _> = tuples.into_iter().collect();
    println!("{:?}", m);
}