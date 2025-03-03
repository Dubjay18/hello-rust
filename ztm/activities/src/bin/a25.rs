// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter


trait Shape {
    fn perimeter(&self) -> i32;
}

struct Square {
    side: i32
}
    impl Shape for Square {
        fn perimeter(&self) -> i32 {
            self.side * 4
        }
    }

struct Triangle {
    a: i32,
    b: i32,
    c: i32
}
    impl Shape for Triangle {
        fn perimeter(&self) -> i32 {
            self.a + self.b + self.c
        }
    }

fn print_perimeter(shape: &impl Shape) {
    println!("Perimeter: {}", shape.perimeter());
}
fn main() {

    let square = Square {
        side: 4
    };
    let triangle = Triangle {
        a: 3,
        b: 4,
        c: 5
    };
    print_perimeter(&square);
    print_perimeter(&triangle);
}
