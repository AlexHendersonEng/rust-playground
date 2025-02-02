/*
    Format strings: Used to variables into strings in a easy and strucutured manner.
*/

fn main() {
    // Define variables
    let name = "Alice";
    let age = 30;

    // Basic formatting
    println!("Hello, {}!", name);

    // Positional arguments
    println!("{0} is {1} years old.", name, age);

    // Named arguments
    println!("{name} is {age} years old.", name = name, age = age);

    // Formatting numbers
    let num = 42;
    println!("Decimal: {} | Binary: {:b} | Hex: {:x} | Octal: {:o}", num, num, num, num);

    // Padding and alignment
    println!("Right align: {:>10}", name);
    println!("Left align: {:<10}", name);
    println!("Center align: {:^10}", name);

    // Zero-padding
    println!("Zero-padded: {:06}", num);

    // Debug formatting (useful for printing complex types)
    let tuple = (3, "hello");
    println!("Debug: {:?}", tuple);

    // Escaping braces
    println!("Use curly braces: {{ and }}");

    // Using `format!` instead of `println!`
    let formatted = format!("{} is {} years old.", name, age);
    println!("{}", formatted);
}