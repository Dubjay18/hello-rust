// Topic: Browsing standard library documentation
//
// Requirements:
// * Print a string in lowercase and uppercase
//
// Notes:
// * Utilize standard library functionality to
//   transform the string to lowercase and uppercase
// * Use 'rustup doc' in a terminal to open the standard library docs
// * Navigate to the API documentation section
// * Search for functionality to transform a string (or str)
//   to uppercase and lowercase
//   * Try searching for: to_uppercase, to_lowercase

fn turn_to_uppercase(s: &str) -> String {
    s.to_uppercase()
}

fn turn_to_lowercase(s: &str) -> String {
    s.to_lowercase()
}
fn main() {
    let s = "Hello, World!";
    println!("Uppercase: {}", turn_to_uppercase(s));
    println!("Lowercase: {}", turn_to_lowercase(s));
}
