// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct GItem {
    quantity: i32,
    id: i32
}


fn display_quantity(item: &GItem) {
    println!("{:?}",item.quantity)
}

fn display_id(g_item: &GItem) {
    println!("{:?}",g_item.id)
}
fn main() {
    let gi = GItem {
        quantity: 10,
        id: 110,
    };
    display_quantity(&gi);
    display_id(&gi)

}
