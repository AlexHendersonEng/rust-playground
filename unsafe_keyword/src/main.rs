/*
    unsafe keyword: Allows you to perform operations that bypass the safety guarantees normally
                    enforced by the Rust compiler. Unsafe code is sometimes necessary for low-level
                    operations like working with raw pointers, interfacing with C code, or
                    accessing hardware registers.
*/

fn main() {
    // Simple example of dereferencing a raw pointer using unsafe
    let x: i32 = 42;
    let raw_ptr: *const i32 = &x; // Creating a raw pointer to x

    // Dereferencing a raw pointer is unsafe, so we use `unsafe` block
    unsafe {
        println!("Value at raw_ptr: {}", *raw_ptr);
    }

    // Mutable raw pointer example
    let mut y: i32 = 10;
    let raw_mut_ptr: *mut i32 = &mut y; // Creating a mutable raw pointer to y

    // Modifying the value through the raw pointer
    unsafe {
        *raw_mut_ptr = 99;
    }

    println!("Modified value of y: {}", y);

    // Calling an unsafe function
    unsafe fn dangerous_function() {
        println!("This is an unsafe function!");
    }

    // To call an unsafe function, you need to use an `unsafe` block
    unsafe {
        dangerous_function();
    }
}
