

// Converts Fahrenheit to Celsius
fn fahrenheit_to_celsius(f: f64) -> f64 {
    5.0/9.0 * (f - 32.0)
}

// Converts Celsius to Fahrenheit
fn celsius_to_fahrenheit(c: f64) -> f64 {
    (9.0 * c)/5.0 + 32.0
}

// Computes the nth Fibonacci number
fn fibonacci(n: i32) -> i32 {
  match n {
    0 => 0,
    1 => 1,
    _ => fibonacci(n-1) + fibonacci(n-2)
  }
}

fn main() {
    println!("Hello, world!");

    // Example usage of the conversion functions
    let f = 100.0;
    let c = fahrenheit_to_celsius(f);
    println!("{} Fahrenheit is {} Celsius", f, c);

    let c = 37.0;
    let f = celsius_to_fahrenheit(c);
    println!("{} Celsius is {} Fahrenheit", c, f);

    // Example usage of the Fibonacci function
    let n = 10;
    let fib = fibonacci(n);
    println!("The {}th Fibonacci number is {}", n, fib);
    let gifts = [
        "a Partridge in a Pear Tree",
        "two Turtle Doves",
        "three French Hens",
        "four Calling Birds",
        "five Gold Rings",
        "six Geese a Laying",
        "seven Swans a Swimming",
        "eight Maids a Milking",
        "nine Ladies Dancing",
        "ten Lords a Leaping",
        "eleven Pipers Piping",
        "twelve Drummers Drumming",
    ];

    let ordinals = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth",
    ];

    for day in 0..12 {
        println!(
            "On the {} day of Christmas, my true love gave to me:",
            ordinals[day]
        );

        for gift in (0..=day).rev() {
            if gift == 0 && day > 0 {
                print!("and ");
            }
            println!("{}", gifts[gift]);
        }

        println!(); // Add a blank line between verses
    }

}
