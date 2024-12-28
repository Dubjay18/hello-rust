use tokio;
use std::{time};


async fn print_odd(){
    let total = 10;
    let mut count = 0;
    le mut odd = 1;
    let three_seconds = time::Duration::from_millis(3000);
    while count < total {
        println!("Odd number: {}", odd);
        odd += 2;
        count += 1;
        tokio::time::sleep(three_seconds).await;
    }
}
fn main() {


}

