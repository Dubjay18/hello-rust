use std::io;
// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)


enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

fn print_power_message(state: PowerState) {
    match state {
        PowerState::Off => println!("Turning off"),
        PowerState::Sleep => println!("Going to sleep"),
        PowerState::Reboot => println!("Rebooting"),
        PowerState::Shutdown => println!("Shutting down"),
        PowerState::Hibernate => println!("Hibernating"),
    }
}

fn get_input() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
fn main() {
    println!("Enter power state:");
    let input = get_input().expect("Failed to read line");
    let state = match input.to_lowercase().as_str() {
        "off" => PowerState::Off,
        "sleep" => PowerState::Sleep,
        "reboot" => PowerState::Reboot,
        "shutdown" => PowerState::Shutdown,
        "hibernate" => PowerState::Hibernate,
        _ => {
            println!("Invalid power state");
            return;
        }
    };
    print_power_message(state);
}

