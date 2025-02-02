/*
    Ownership: A set of rules that govern how a Rust program manages memory.
*/

fn main() {
    // Define string variable
    let s1 = String::from("Hello, ownership!");
    
    // Ownership of s1 is moved to s2
    let s2 = s1;
    
    // This will cause an error because s1 no longer owns the data
    // println!("{}", s1); // Uncommenting this line will give a compile-time error

    // But we can use s2, because it now owns the data
    println!("{}", s2);
}