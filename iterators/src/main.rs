/*
    Iterators: Allow sequential access to elements in a collection. They provide a way to process
               data lazily, meaning elements are generated only when needed. The 'Iterator' trait
               requires implementing the 'next()' method, which returns 'Some(Item)' for the next
               element or `None` when iteration is complete.
*/

// Define a simple counter struct that will implement an iterator
struct Counter {
    count: u32, // Keeps track of the current count
}

impl Counter {
    // Constructor function to create a new Counter instance
    fn new() -> Counter {
        Counter { count: 0 } // Initialize count to zero
    }
}

// Implement the Iterator trait for Counter
impl Iterator for Counter {
    type Item = u32; // The iterator will yield `u32` values

    fn next(&mut self) -> Option<Self::Item> {
        // Check if the count is less than 5
        if self.count < 5 {
            self.count += 1; // Increment the count
            Some(self.count) // Return the new count wrapped in Some
        } else {
            None // Stop iteration when count reaches 5
        }
    }
}

fn main() {
    // Create a new Counter instance
    let counter = Counter::new();
    
    // Iterate over the counter and print each value
    for val in counter {
        println!("Counter: {}", val);
    }
    
    // Create a vector of numbers
    let numbers = vec![1, 2, 3, 4, 5];
    
    // Use the `map` iterator adapter to square each number
    let squared: Vec<_> = numbers.iter().map(|x| x * x).collect();
    println!("Squared numbers: {:?}", squared);
    
    // Use the `filter` iterator adapter to keep only even numbers
    let even_numbers: Vec<_> = numbers.into_iter().filter(|x| x % 2 == 0).collect();
    println!("Even numbers: {:?}", even_numbers);
}
