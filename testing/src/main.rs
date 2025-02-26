/*
    Testing: Uses special functions that check if your code works as expected. They
             typically use the 'assert!', 'assert_eq!', or 'assert_ne!' macros to
             verify conditions. Rust provides a built-in test framework, and tests
             are run using the 'cargo test' command.
*/

// Enable test functionality using Rust's built-in test framework.
#[cfg(test)] // This ensures the following module is only compiled when running tests.
mod tests {
    // The 'test' attribute marks a function as a test case.
    use super::*; // This allows tests to access functions from the main module.

    /// A simple function that adds two numbers.
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    /// Test case to check if 'add' function works correctly.
    #[test] // This macro marks this function as a test.
    fn test_add() {
        assert_eq!(add(2, 3), 5); // Check if 2 + 3 equals 5
    }

    /// Another test case to check a failure scenario.
    #[test]
    fn test_add_negative() {
        assert_eq!(add(-1, -1), -2); // Check if -1 + -1 equals -2
    }
}

