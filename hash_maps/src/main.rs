/*
    Hash maps: A way to store key value pairs.
*/

use std::collections::HashMap;

fn main() {
    // 1. Creating a new HashMap
    let mut map = HashMap::new();

    // 2. Inserting values
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    println!("HashMap: {:?}", map);

    // 3. Accessing values
    if let Some(value) = map.get("two") {
        println!("Value for 'two': {}", value);
    }

    // 4. Iterating over key-value pairs
    for (key, value) in &map {
        println!("Key: {}, Value: {}", key, value);
    }

    // 5. Checking for a key and updating its value
    map.entry("four").or_insert(4);
    println!("Updated HashMap: {:?}", map);

    // 6. Removing a key
    map.remove("one");
    println!("After removal: {:?}", map);
}

