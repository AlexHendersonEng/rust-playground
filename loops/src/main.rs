/*
    Loops: A way of executing code multiple times.
*/

fn main() {
    // 1. 'loop': A basic infinite loop
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 5 {
            println!("Breaking the loop after {} iterations", counter);
            break; // Break out of the loop
        }
    }

    // 2. 'while': A loop that runs as long as a condition is true
    let mut number = 0;
    while number < 5 {
        println!("The number is: {}", number);
        number += 1;
    }

    // 3. 'for': A loop that iterates over a range or collection
    for i in 0..5 {
        println!("This is iteration number {}", i);
    }

    // 4. `for` loop with a collection (array in this case)
    let arr = [10, 20, 30, 40, 50];
    for &value in arr.iter() {
        println!("The value is: {}", value);
    }
}