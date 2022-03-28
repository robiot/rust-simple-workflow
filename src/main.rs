fn calculate_difficult_math(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("Hello, world! {}", calculate_difficult_math(5, 5));
}

#[cfg(test)]
mod tests {
    // Importing all functions in the super scope
    use super::*;

    #[test]
    fn test_difficult_math() {
        // Panics if the left value is not equals to the right
        assert_eq!(calculate_difficult_math(10, 5), 15);
    }
}
