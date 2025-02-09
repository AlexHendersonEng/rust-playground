/*
    Vectors: A dynamic, growable array that stores elements of the same type in a contiguous
             memory block. Unlike arrays, vectors can change size at runtime, making them
             useful when the number of elements is unknown in advance. They are part of Rust’s
             collections module and provide methods for adding, removing, and accessing
             elements efficiently.
*/

fn main() {
    // 1. Creating a new empty vector
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("Vector after pushing elements: {:?}", v);

    // 2. Creating a vector with initial values
    let v2 = vec![10, 20, 30, 40];
    println!("Vector with initial values: {:?}", v2);

    // 3. Accessing elements
    println!("First element: {}", v2[0]);

    // Using get() to safely access elements
    if let Some(value) = v2.get(2) {
        println!("Third element using get(): {}", value);
    }

    // 4. Iterating over a vector
    for x in &v2 {
        println!("Value in v2: {}", x);
    }

    // 5. Iterating and modifying elements
    for x in &mut v {
        *x *= 2; // Multiply each element by 2
    }
    println!("Modified v after iteration: {:?}", v);

    // 6. Removing elements
    v.pop();
    println!("Vector after pop(): {:?}", v);

    // 7. Clearing all elements
    v.clear();
    println!("Vector after clear(): {:?}", v);

    // 8. Checking if a vector is empty
    println!("Is v empty? {}", v.is_empty());
}
