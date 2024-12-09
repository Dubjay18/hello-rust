// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    locker: Option<i32>,
    name: String
}

fn main() {
    let student1 = Student{
        locker: Some(23),
        name: "JAy".to_owned()
    };
    println!("Student: {:?}", student1.name);
    match student1.locker {
        Some(locker) => println!("Locker: {:?}", locker),
        None => println!("Locker: None")
    }
}
