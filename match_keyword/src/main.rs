/*
    match keyword: A powerful control flow construct that allows pattern matching on values.
                   It is used to handle different possible cases in a concise and readable way.
*/

fn main() {
    // 1. Matching on an integer
    let number = 2;
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Something else"), // _ is a catch-all pattern
    }

    // 2. Matching on an enum
    enum Color {
        Red,
        Green,
        Blue,
    }
    
    let color = Color::Green;
    match color {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }
    
    // 3. Matching on a tuple
    let point = (0, 5);
    match point {
        (0, y) => println!("On the y-axis at {}", y),
        (x, 0) => println!("On the x-axis at {}", x),
        (x, y) => println!("At point ({}, {})", x, y),
    }
    
    // 4. Matching on an Option<T>
    let some_number: Option<i32> = Some(10);
    match some_number {
        Some(value) => println!("Got a value: {}", value),
        None => println!("Got nothing"),
    }
    
    // 5. Matching with guards (extra conditions)
    let age = 18;
    match age {
        x if x < 18 => println!("Underage"),
        18 => println!("Exactly 18"),
        _ => println!("Over 18"),
    }
    
    // 6. Matching with multiple patterns
    let day = "Saturday";
    match day {
        "Saturday" | "Sunday" => println!("Weekend"),
        _ => println!("Weekday"),
    }
}
