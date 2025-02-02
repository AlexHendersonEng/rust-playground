/*
    Data types: Every value is of a certain data type, which tells Rust what kind
                of data is being specified so it knows how to work with that data.
*/

fn main() {
    // Scalar types
    let integer: i32 = 42; // Signed integer
    let unsigned: u32 = 100; // Unsigned integer
    let floating: f64 = 3.14; // Floating-point number
    let boolean: bool = true; // Boolean value
    let character: char = 'A'; // Character type

    // Compound types
    let tuple: (i32, f64, char) = (10, 2.5, 'Z'); // Tuple
    let (x, y, z) = tuple; // Destructuring

    let array: [i32; 3] = [1, 2, 3]; // Array
    let first_element = array[0]; // Accessing array elements
    let array1 = [3; 5]; // Initialise array of length 5 full of 3's

    // String types
    let str_literal: &str = "Hello, world!"; // String slice (immutable)
    let mut string_obj: String = String::from("Hello"); // Growable string
    string_obj.push_str(", Rust!");
    
    // Print values
    println!("Integer: {}", integer);
    println!("Unsigned: {}", unsigned);
    println!("Floating: {}", floating);
    println!("Boolean: {}", boolean);
    println!("Character: {}", character);
    println!("Tuple: ({}, {}, {})", x, y, z);
    println!("Array: {:?}", array);
    println!("Array first element: {}", first_element);
    println!("Another array: {:?}", array1);
    println!("String literal: {}", str_literal);
    println!("String object: {}", string_obj);
}
