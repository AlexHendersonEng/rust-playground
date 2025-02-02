/*
    Variables: Named storage for data that porgrams can manipulate.
*/

fn main() {
    // Define an immutable variable
    let x = 5;
    // x = 6; // This will cause an error because x is immutable
    println!("The value of x is: {x}");

    // Define a shadowed variable
    let x = x + 1;
    println!("The value of x is: {x}");

    // Define a mutable variable
    let mut y = 5;
    println!("The value of y is: {y}");
    y = 6;
    println!("Value of y changed: {y}");

    // Define a immutable variables with type annotations
    let z: i32 = 5;
    println!("The value of z is: {z}");

    // Define a constant which is always immutable and must be type annotated
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {MAX_POINTS}");
}
