/*
    Concurrency: The ability of a program to execute multiple tasks independently, potentially
                 overlapping in execution. Rust provides several ways to achieve concurrency,
                 ensuring memory safety without data races.
    
                 Key Concurrency primitives:
                 1. Threads: The simplest way to create concurrent execution.
                 2. Message Passing: Using channels for safe communication between threads.
                 3. Shared State: Using 'Arc' (Atomic Reference Counting) and 'Mutex' for
                                  synchronized access to shared data.
                 4. Async/Await: Non-blocking concurrency for I/O-bound tasks.
*/

// Importing required modules
use std::thread;
use std::sync::{Arc, Mutex, mpsc};
use std::time::Duration;
use tokio::time;

// 1. Using Threads
fn basic_thread_example() {
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("Thread: Count {}", i);
            thread::sleep(Duration::from_millis(500));
        }
    });
    handle.join().unwrap(); // Ensure the thread completes before the function exits
}

// 2. Message Passing with Channels
fn message_passing_example() {
    let (tx, rx) = mpsc::channel(); // Create a channel (tx = sender, rx = receiver)

    thread::spawn(move || {
        let message = "Hello from thread";
        tx.send(message).unwrap(); // Send the message
    });
    
    let received = rx.recv().unwrap(); // Receive the message
    println!("Received: {}", received);
}

// 3. Shared State with Mutex and Arc
fn shared_state_example() {
    let counter = Arc::new(Mutex::new(0)); // Thread-safe reference-counted integer
    let mut handles = vec![];

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // Lock the mutex to access the data
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final counter value: {}", *counter.lock().unwrap());
}

// 4. Async/Await (Requires Tokio)
#[tokio::main]
async fn async_example() {
    async fn task(name: &str, duration: u64) {
        println!("{} started", name);
        time::sleep(Duration::from_secs(duration)).await;
        println!("{} finished", name);
    }

    let task1 = task("Task 1", 2);
    let task2 = task("Task 2", 1);
    
    tokio::join!(task1, task2); // Run tasks concurrently
}

fn main() {
    println!("Running basic_thread_example...");
    basic_thread_example();
    
    println!("\nRunning message_passing_example...");
    message_passing_example();
    
    println!("\nRunning shared_state_example...");
    shared_state_example();
    
    println!("\nRunning async_example...");
    async_example();
}
