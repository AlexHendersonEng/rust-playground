/*
    Macros: A powerful way to write code that writes other code (also known as metaprogramming).
            They allow you to define patterns that expand into code when invoked.
*/

// Define a simple macro named 'say_hello!'
macro_rules! say_hello {
    // Pattern to match with no arguments
    () => {
        println!("Hello, World!");
    };

    // Pattern to match with one argument
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
}

fn main() {
    // Call the macro with no arguments
    say_hello!();

    // Call the macro with one argument
    say_hello!("Alice");
}
