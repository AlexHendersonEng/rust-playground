/*
    Crates: The smallest unit of code compilation in Rust. It can be a binary crate 
            (executable program) or a library crate (reusable code). A 'Cargo.toml'
            file defines the crate dependencies.

    Modules: Organises code inside a crate keeping code structured and preventing
             namespace conflicts.
    
    Packages: A collection of one or more crates. A 'Cargo.toml' file manages the
              package, specifying dependencies and metadata.
*/

// Define a module named 'math'
mod math {
    // Define a public function 'add'
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    // Define a public function 'multiply'
    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }
}

fn main() {
    // Use module functions
    let sum = math::add(5, 3);
    let product = math::multiply(4, 2);

    // Print results
    println!("Sum: {}", sum);
    println!("Product: {}", product);
}
