use chrono::prelude::*;
// Topic: External crates
//
// Requirements:
// * Display the current date and time
//
// Notes:
// * Use the `chrono` crate to work with time
// * (OPTIONAL) Read the documentation section `Formatting and Parsing`
//   for examples on how to create custom time formats

fn main() {
    let local: DateTime<Local> = Local::now();
    println!("Local time: {}", local.format("%Y-%m-%d %H:%M:%S").to_string());
}
