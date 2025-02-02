/*
    Functions: A block of code that can be called repeatedly in a program.
*/

// Basic function with parameters and return type
fn add(a: i32, b: i32) -> i32 {
    a + b // Implicit return (no semicolon)
}

// Function with explicit return
fn multiply(a: i32, b: i32) -> i32 {
    return a * b; // Explicit return with semicolon
}

// Function with no parameters and no return value
fn say_hello() {
    println!("Hello!");
}

// Function with a return type but no parameters
fn get_number() -> i32 {
    42
}

// Function returning a tuple
fn swap(x: i32, y: i32) -> (i32, i32) {
    (y, x)
}

// Function using closures
fn closure_example() {
    let square = |x: i32| x * x; // Closure that squares a number
    println!("Square of 4 is {}", square(4));
}

fn main() {
    // Calling a function with no return value
    say_hello();

    // Calling a function with multiple arguments and a implicit return value
    let sum = add(2, 3);
    println!("Sum: {}", sum);

    // Calling a function with multiple arguments and a explicit return value
    let product = multiply(4, 5);
    println!("Product: {}", product);

    // Calling a function wih no arguments but one return value
    let num = get_number();
    println!("Number: {}", num);

    // Calling a function with multiple arguments that returns a tuple
    let (a, b) = swap(10, 20);
    println!("Swapped: {} {}", a, b);

    // Calling a function with a closure
    closure_example();
}

