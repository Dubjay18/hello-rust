// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn main() {
    let mut furniture = HashMap::new();
    furniture.insert("Chairs", 5);
    furniture.insert("Beds", 3);
    furniture.insert("Tables", 2);
    furniture.insert("Couches", 0);

    for (item, count) in &furniture {
        if *count == 0 {
            println!("{:?} out of stock", item);
        } else {
            println!("{:?} {:?} in stock", item, count);
        }
    }
    let total: i32 = furniture.values().sum();
    println!("Total items in stock: {:?}", total);
}
