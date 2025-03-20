/*
    Bindings (@): Allows for the creation of a variable that stores a value while simultaneously
                  matching that value to a pattern. This can be useful when you want to capture
                  a matched value for later use without breaking the pattern.
*/

fn main() {
    let number = 5;

    // Match against different ranges and use @ to bind a value
    match number {
        // Bind the value to 'n' and check if it's in the range 1..=5
        n @ 1..=5 => println!("Number {} is between 1 and 5", n),
        
        // Bind the value to 'n' and check if it's greater than 5
        n @ 6..=10 => println!("Number {} is between 6 and 10", n),
        
        // Any other value
        _ => println!("Number is outside the expected range"),
    }

    // Define a simple struct for demonstration
    struct Point {
        x: i32,
        y: i32,
    }

    // Complex Pattern with Structs
    let point = Point { x: 5, y: 10 };

    match point {
        // Bind the x value to 'a' while matching
        Point { x: a @ 1..=5, y } => println!("x is in range, x = {}, y = {}", a, y),

        // Default case
        Point { x, y } => println!("Point: x = {}, y = {}", x, y),
    }
}