// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.

struct Bill {
    name: String,
    amount: f64,
}

struct BillManager {
    store: Vec<Bill>,
}

impl BillManager {
    fn new() -> Self {
        BillManager {
            store: vec![],
        }
    }

    fn add_to_store(&mut self, bill: Bill) {
        self.store.push(bill);
    }

    fn remove_from_store(&mut self, name: &str) {
        self.store.retain(|bill| bill.name != name);
    }
    fn add_bill(&mut self, name: &str, amount: f64) {
        let bill = Bill {
            name: name.to_string(),
            amount,
        };
        self.add_to_store(bill);
    }

    fn view_bills(&self) {
        for bill in &self.store {
            println!("{}: ${}", bill.name, bill.amount);
        }
    }

    fn remove_bill(&mut self, name: &str) {
        self.remove_from_store(name);
    }

    fn edit_bill(&mut self, name: &str, amount: f64) {
        for bill in &mut self.store {
            if bill.name == name {
                bill.amount = amount;
            }
        }
    }
}



fn main() {
    println!("Bill Manager");
    let mut manager = BillManager::new();
    loop {
        println!("Menu:");
        println!("1. Add bill");
        println!("2. View bills");
        println!("3. Remove bill");
        println!("4. Edit bill");
        println!("5. Exit");
        println!("Enter the number of your choice:");
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice");
                continue;
            }
        };
        match choice {
            1 => {
                println!("Enter bill name:");
                let mut name = String::new();
                std::io::stdin().read_line(&mut name).expect("Failed to read line");
                let name = name.trim();
                println!("Enter bill amount:");
                let mut amount = String::new();
                std::io::stdin().read_line(&mut amount).expect("Failed to read line");
                let amount: f64 = match amount.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid amount");
                        continue;
                    }
                };
                manager.add_bill(name, amount);
            }
            2 => manager.view_bills(),
            3 => {
                println!("Enter bill name:");
                let mut name = String::new();
                std::io::stdin().read_line(&mut name).expect("Failed to read line");
                let name = name.trim();
                manager.remove_bill(name);
            },
            4 => {
                println!("Enter bill name:");
                let mut name = String::new();
                std::io::stdin().read_line(&mut name).expect("Failed to read line");
                let name = name.trim();
                println!("Enter new bill amount:");
                let mut amount = String::new();
                std::io::stdin().read_line(&mut amount).expect("Failed to read line");
                let amount: f64 = match amount.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid amount");
                        continue;
                    }
                };
                manager.edit_bill(name, amount);
            },
            5 => break,
            _ => {}
        }
    }
}
