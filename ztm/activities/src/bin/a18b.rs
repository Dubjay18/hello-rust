// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

enum Role {
    Maintenance,
    Marketing,
    Manager,
    LineSupervisor,
    KitchenStaff,
    AssemblyTechnician
}

struct Employee {
    role: Role,
    employed: bool
}
impl Employee {
    fn new(role: Role, employed: bool) -> Self {
        Self {
            role,
            employed
        }
    }
    fn access_building(&self) -> Result<(),String> {
        match self.role {
            Role::Maintenance | Role::Marketing | Role::Manager => {
                if self.employed {
                    Ok(())
                }else{
                    Err("You are terminated".to_owned())
                }
            },
            _ => {
                Err("You are not allowed".to_owned())
            }
        }
    }
}



fn main() {
    let employee = Employee::new(Role::Maintenance, true);
    match employee.access_building() {
        Ok(_) => println!("You can access the building"),
        Err(e) => println!("Error: {}", e)
    }

    let terminated_employee = Employee::new(Role::LineSupervisor, false);
    match terminated_employee.access_building() {
        Ok(_) => println!("You can access the building"),
        Err(e) => println!("Error: {}", e)
    }
}
