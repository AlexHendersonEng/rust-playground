// Create local name binding for 'std::io'
use std::io;

fn main() {
    // Ask use for their name
    println!("Enter your name:");
    
    // Define mutable empty string variable
    let mut name = String::new();
    
    // Read user input variable and throw error if something goes wrong
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    
    // Say hello to user
    println!("Hello, {}!", name.trim());
}

