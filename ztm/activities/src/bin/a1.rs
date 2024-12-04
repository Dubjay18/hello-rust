// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn first_name() -> String {
  return "John".to_string();

}
fn last_name() -> String {
    return "Doe".to_string();
}

fn main() {
    println!("First name: {}", first_name());
    println!("Last name: {}", last_name());
}
