/*
    Smart Pointers: Data structures that not only act as pointers but also manage memory automatically. 
                    Rust provides several smart pointers, including 'Box<T>', 'Rc<T>', 'Arc<T>', and 'RefCell<T>'. 
                    These allow for more flexible memory management while maintaining Rust's safety guarantees.
    
    Overview of Smart Pointers:
        - 'Box<T>': Used for heap allocation when a value's size is unknown at compile time or when moving
                    large structures.
        - 'Rc<T>' (Reference Counted Pointer): Enables multiple ownership in single-threaded environments.
        - 'Arc<T>' (Atomic Reference Counted Pointer): Similar to 'Rc<T>' but used in multi-threaded scenarios.
        - 'RefCell<T>': Allows for interior mutability when you need to mutate borrowed data.
*/

// Import necessary libraries
use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;

// Example 1: Box<T> - Storing a value on the heap
fn box_example() {
    let boxed_value = Box::new(42); // Allocates 42 on the heap
    println!("Boxed value: {}", boxed_value);
}

// Example 2: Rc<T> - Reference counting for shared ownership
fn rc_example() {
    let rc_value = Rc::new(42); // Create a reference-counted integer
    let rc_clone = Rc::clone(&rc_value); // Increase reference count
    
    println!("Reference Count: {}", Rc::strong_count(&rc_value));
    println!("Rc value: {}", rc_value);
    println!("Cloned Rc value: {}", rc_clone);
} // rc_clone and rc_value go out of scope, ref count drops to 0

// Example 3: Arc<T> - Thread-safe reference counting
fn arc_example() {
    let arc_value = Arc::new(42);
    let arc_clone = Arc::clone(&arc_value);
    
    println!("Arc value: {}", arc_value);
    println!("Cloned Arc value: {}", arc_clone);
} // Both references drop when scope ends

// Example 4: RefCell<T> - Interior mutability
fn refcell_example() {
    let refcell_value = RefCell::new(42);
    
    // Borrow mutably and modify
    *refcell_value.borrow_mut() += 1;
    
    println!("Modified RefCell value: {}", refcell_value.borrow());
} // Borrowing checks happen at runtime

fn main() {
    println!("--- Box<T> Example ---");
    box_example();
    
    println!("\n--- Rc<T> Example ---");
    rc_example();
    
    println!("\n--- Arc<T> Example ---");
    arc_example();
    
    println!("\n--- RefCell<T> Example ---");
    refcell_example();
}
