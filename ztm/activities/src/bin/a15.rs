// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Ticket {
    Backstage(String,f64),
    Vip(String,f64),
    Standard(f64)
}
fn main() {
let tickets = vec![Ticket::Backstage("jay".to_owned(),40.0),
                   Ticket::Vip("tj".to_owned(),40.0),
                   Ticket::Standard(40.0),
];

    for ticket in tickets {
        match ticket {
            Ticket::Backstage(holder,price)=> println!("Holder: {:?}, Price: {:?}",holder,price),
            Ticket::Vip(holder,price)=> println!("Holder: {:?}, Price: {:?}",holder,price),
            Ticket::Standard(price)=> println!("Price: {:?}",price),
        }
    }

}
