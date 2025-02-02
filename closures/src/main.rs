/*
    Closures: Anonymous functions that can capture variables from their surrounding scope.
              They are defined using the |args| { body } syntax and can be stored in variables,
              passed as arguments, or returned from functions. Rust supports three closure
              traits: Fn, FnMut, and FnOnce, depending on how they capture and use captured variables.
*/

fn main() {
    // 1. Assigning a closure to a variable
    let add_one = |x: i32| x + 1;
    println!("1 + 1 = {}", add_one(1));

    // 2. Using a closure directly (inline)
    println!("2 + 2 = {}", (|x: i32| x + 2)(2));

    // 3. A closure that captures a variable from the environment
    let factor = 3;
    let multiply = |x: i32| x * factor;
    println!("3 * 3 = {}", multiply(3));

    // 4. A mutable closure (modifies internal state)
    let mut counter = 0;
    let mut increment = || {
        counter += 1;
        counter
    };
    println!("Increment: {}", increment());
    println!("Increment: {}", increment());

    // 5. A closure passed to a function
    fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
        f(x)
    }
    println!("5 + 3 = {}", apply(|x| x + 3, 5));

    // 6. A closure with multiple parameters
    let sum = |a: i32, b: i32| a + b;
    println!("4 + 6 = {}", sum(4, 6));
}