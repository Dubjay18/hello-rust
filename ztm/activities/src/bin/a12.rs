// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics


enum Color {
    Red,
    Black,
    BLue

}

impl Color {
    fn print(&self) {
       match self {
            Color::Black => println!("black"),
            Color::BLue => println!("blue"),
            Color::Red => println!("red")
        }

    }
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64
}

impl Dimensions {
    fn print(&self) {
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);

    }
}
struct Box {
    weight: f64,
    dimensions: Dimensions,
    color: Color
}

impl Box {
    fn create(color: Color,dimensions:Dimensions,weight: f64) -> Self {
        Self{
            color,
            dimensions,
            weight
        }
    }

    fn print_dets(&self) {
      self.dimensions.print();
        self.color.print();
        println!("{}",self.weight)
    }
}


fn main() {
    let dimensions = Dimensions{
        width: 12.0,
        height: 13.0,
        depth: 60.0
    };
    let new_box = Box::create(Color::Black,dimensions,60.7);
    new_box.print_dets()
}
