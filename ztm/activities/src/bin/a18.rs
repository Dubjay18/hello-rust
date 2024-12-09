// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

struct Customer {
    name: String,
    age: i32
}

impl Customer {
    fn new(name: String,age: i32) -> Self {
        Self {
            name,
            age
        }
    }

    fn check_purchase(&self) -> Result<Self,String> {
        if self.age >= 21 {
            Ok(Self{
                name: self.name.clone(),
                age: self.age
            })
        }else{
            Err("You are not up to 21".to_owned())
        }
    }

}

fn main() {
    let customer = Customer::new("prince".to_owned(), 20);
    match customer.check_purchase() {
        Ok(_) => println!("You can make a purchase"),
        Err(e) => println!("Error: {}", e)
    }

}
