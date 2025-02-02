/*
    Conditional statements: Allow for the control flow of a program to be dictated by conditions.
*/

fn main() {
    // Declare variable
    let x = 10;

    // 1. Basic if-else
    if x > 5 {
        println!("x is greater than 5");
    } else {
        println!("x is less than or equal to 5");
    }

    // 2. if, else if, else
    if x < 5 {
        println!("x is less than 5");
    } else if x == 10 {
        println!("x is exactly 10");
    } else {
        println!("x is greater than 5 but not 10");
    }

    // 3. Using if in a let binding (expression-based if)
    let result = if x % 2 == 0 {
        "even"
    } else {
        "odd"
    };
    println!("x is {}", result);

    // 4. Using match for pattern matching (alternative to complex if-else chains)
    match x {
        1 => println!("x is 1"),
        2..=5 => println!("x is between 2 and 5"),
        10 => println!("x is exactly 10"),
        _ => println!("x is something else"),
    }
}
