// Bringing an entire module into scope
use std::collections;

// Bringing a specific item into scope
use std::collections::HashMap;

// Bringing multiple items from the same module
use std::collections::{HashSet, VecDeque};

// Using a wildcard to bring everything from a module
use std::io::*;

// Renaming an import
use std::collections::BTreeMap as Map;

// Nested use statements
use std::{cmp::Ordering, fs::File};

fn main() {
    // Example usages
    let mut map: HashMap<i32, &str> = HashMap::new();
    map.insert(1, "one");
    println!("{:?}", map);

    let mut set: HashSet<i32> = HashSet::new();
    set.insert(2);
    println!("{:?}", set);

    let _file = File::open("test.txt");
}

