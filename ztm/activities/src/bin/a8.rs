// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum flavour {
    Orange,
    Apple,
    Mango,
    Pineapple,
    Grape,
    Strawberry,
    Peach,
    Lemon,
}
struct Drink {
    flavour: flavour,
    ounces: f64,
}

fn print_drink(drink: Drink) {
    match drink.flavour {
        flavour::Orange => println!("Orange"),
        flavour::Apple => println!("Apple"),
        flavour::Mango => println!("Mango"),
        flavour::Pineapple => println!("Pineapple"),
        flavour::Grape => println!("Grape"),
        flavour::Strawberry => println!("Strawberry"),
        flavour::Peach => println!("Peach"),
        flavour::Lemon => println!("Lemon"),
    }
    println!("{:?} ounces", drink.ounces);
}

fn main() {
    let my_drink = Drink {
        flavour: flavour::Orange,
        ounces: 12.0,
    };
    print_drink(my_drink);
}
