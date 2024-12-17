// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    Some(a / b)
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{} {}", first, second)
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_clamp() {
        let result = clamp(9,2,5);
        let result2 = clamp(1,2,5);
        assert_eq!(result, 5);
        assert_eq!(result2, 2);
    }

    #[test]
    fn test_div() {
        let result = div(10,2);
        let result2 = div(10,0);
        assert_eq!(result, Some(5));
        assert_eq!(result2, None);
    }

    #[test]
    fn test_concat() {
        let result = concat("Hello", "World");
        let result2 = concat("Goodbye", "World");
        assert_eq!(result, "Hello World");
        assert_eq!(result2, "Goodbye World");
    }
}