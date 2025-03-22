/*
    static keyword: in Rust is used to define variables or constants that have a
                    fixed memory location and live for the entire duration of the program.
*/

// Define a static variable with a string literal
static GREETING: &str = "Hello, world!";

// Define a static variable with a constant integer value
static MAX_USERS: u32 = 1000;

// Define a mutable static variable (requires unsafe block to modify)
static mut COUNTER: u32 = 0;

// Function to print static variables
fn print_static_values() {
    println!("GREETING: {}", GREETING);
    println!("MAX_USERS: {}", MAX_USERS);
}

// Function to demonstrate modifying a mutable static variable
fn increment_counter() {
    // Unsafe block is required to access and modify mutable static variables
    unsafe {
        COUNTER += 1;
        println!("COUNTER after increment: {}", COUNTER);
    }
}

fn main() {
    // Print values of static variables
    print_static_values();

    // Increment and print the counter
    increment_counter();
    increment_counter();

    // Accessing mutable static variable requires 'unsafe' block even for reading
    unsafe {
        println!("Final COUNTER value: {}", COUNTER);
    }
}
