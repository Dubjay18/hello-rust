// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function



// * Use a struct for a persons age, name, and favorite color
struct Person {
    age: i32,
    name: String,
    favorite_color: String
}
fn main() {
    let people = vec![
    Person{
        age: 19,
        name: String::from("prince"),
        favorite_color: String::from("red")
    },
    Person{
        age: 20,
        name: String::from("sung jin woo"),
        favorite_color: String::from("blue")
    },
    Person{
         age: 5,
         name: String::from("jennie"),
         favorite_color: String::from("blue")
    }
    ];

    for person in people {
        if person.age <= 10 {
            println!("name: {:?}, favorite color: {:?}", person.name, person.favorite_color)
        }
    }
}
