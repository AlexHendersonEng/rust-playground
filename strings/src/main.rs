/*
    Strings: A way to store text in a variable.
*/

fn main() {
    // 1. String Literal (Immutable, fixed in memory)
    let s1: &str = "Hello, world!";
    println!("String Literal: {}", s1);

    // 2. String Type (Heap-allocated, mutable)
    let mut s2 = String::from("Hello");
    s2.push_str(", world!"); // Append text
    println!("String Object: {}", s2);

    // 3. Converting a string literal to a String
    let s3 = "Rust".to_string();
    println!("Converted String: {}", s3);

    // 4. Concatenation with + operator (Requires ownership transfer)
    let s4 = String::from("Hello") + " Rust!";
    println!("Concatenated: {}", s4);

    // 5. Using format! macro (More flexible concatenation)
    let s5 = format!("{} {}", "Hello", "Rust!");
    println!("Formatted: {}", s5);

    // 6. Iterating over characters
    for c in "Rust".chars() {
        println!("Character: {}", c);
    }

    // 7. String Slicing (Extracting part of a string slice)
    let s6 = &s1[0..5]; // "Hello"
    println!("Sliced: {}", s6);
}
