

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
                   3 * 7 * 11 * 13 * 19),
               3 * 11);
}

use std::env;
use std::str::FromStr;
fn main() {

let mut numbers = Vec::new();
    let args = env::args().skip(1);
    for arg in args {
       let result = u64::from_str(&arg);
         match result {
              Ok(n) => numbers.push(n),
              Err(e) => {
                  eprintln!("error: {}", e);
                    std::process::exit(1);
              },
         }

    }
    if numbers.len() == 0 {
        eprintln!("USAGE: gcd num1 nu2....");
        std::process::exit(1)
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }
    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

