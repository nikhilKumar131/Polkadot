// Declare that this is a no_std library
#![no_std]

// A simple function that adds two numbers
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn mul(a: i32, b: i32) -> i32 {
    a * b
}

pub fn div(a: i32, b: i32) -> i32 {
    a / b
}

pub fn rem(a: i32, b: i32) -> i32 {
    a % b
}

// If needed, you can add unit tests here, which will use std
#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn it_adds_two_numbers() {
        assert_eq!(add(2, 3), 5);
    }
}
