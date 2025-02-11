/*
    Error handling: Allows a program to deal with an error in a program by panicking
                    or executing alternate code.
*/

use std::fs::File;
use std::io::{self, Read};

// 1. Using `Result` for explicit error handling
fn read_file() -> Result<String, io::Error> {
    let mut file = File::open("example.txt")?; // `?` propagates the error if any
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// 2. Using `match` to handle errors manually
fn read_file_with_match() {
    let result = File::open("example.txt");
    
    match result {
        Ok(file) => println!("File opened successfully: {:?}", file),
        Err(e) => println!("Failed to open file: {}", e),
    }
}

// 3. Using `unwrap()` to panic on errors
fn read_file_with_unwrap() {
    let file = File::open("example.txt").unwrap(); // Panics if there's an error
    println!("File opened: {:?}", file);
}

// 4. Using `expect()` to panic with a custom message
fn read_file_with_expect() {
    let file = File::open("example.txt").expect("Failed to open example.txt");
    println!("File opened: {:?}", file);
}

// 5. Using `Option` for non-critical errors
fn divide_numbers(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None // Return None instead of crashing
    } else {
        Some(a / b)
    }
}

fn main() {
    // Using Result with `?`
    match read_file() {
        Ok(contents) => println!("File content: {}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }

    // Using match
    read_file_with_match();

    // Using unwrap (will panic if file doesn't exist)
    // read_file_with_unwrap();

    // Using expect (will panic with a message)
    // read_file_with_expect();

    // Using Option for non-fatal errors
    match divide_numbers(10.0, 2.0) {
        Some(result) => println!("Division result: {}", result),
        None => println!("Cannot divide by zero"),
    }
}

