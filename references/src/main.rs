/*
    References: Allow you to access values without taking ownership, enabling efficient
                memory usage and preventing data races. Borrowing comes in two forms:
                immutable borrowing (&T), which allows multiple reads but no modifications,
                and mutable borrowing (&mut T), which grants exclusive access for modification.
                Rust enforces strict borrowing rules at compile time to ensure memory safety.
*/

fn main() {
    // Immutable reference (borrowing)
    let x = 10;
    let ref1 = &x;
    println!("Immutable reference: {}", ref1);
    
    // Mutable reference
    let mut y = 20;
    {
        let ref2 = &mut y;
        *ref2 += 5;
        println!("Mutable reference inside scope: {}", ref2);
    }
    println!("Modified value: {}", y);
}
