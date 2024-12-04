// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print


enum Colour {
    Red,
    Green,
    Blue,
    Yellow,
    Orange,
    Purple,
    Black,
    White,
}

fn print_colour(colour: Colour) {
    match colour {
        Colour::Red => println!("Red"),
        Colour::Green => println!("Green"),
        Colour::Blue => println!("Blue"),
        Colour::Yellow => println!("Yellow"),
        Colour::Orange => println!("Orange"),
        Colour::Purple => println!("Purple"),
        Colour::Black => println!("Black"),
        Colour::White => println!("White"),
    }
}
fn main() {
    let colour = Colour::Red;
    print_colour(colour);
}
