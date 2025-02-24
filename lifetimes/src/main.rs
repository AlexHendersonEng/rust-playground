/*
    Lifetimes: Ensure that references remain valid for the required duration. They prevent
               dangling references and help manage memory safely without garbage collection.
*/

// Basic lifetime annotation in a function
// The function takes two string slices with the same lifetime and returns the longest one.
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

// Struct with a lifetime parameter
// This struct holds a reference to a string slice, ensuring it does not outlive the referenced data.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // Method using lifetime annotation
    // This method prints the stored reference, ensuring its validity.
    fn announce(&self) {
        println!("Attention: {}", self.part);
    }
}

// Lifetime elision (Rust can infer lifetimes)
// Since Rust knows the return value references the input, it automatically assigns a lifetime.
fn first_word(s: &str) -> &str {
    &s[..s.find(' ').unwrap_or(s.len())]
}

fn main() {
    let s1 = "Hello";
    let s2 = "World!";
    
    // Using function with explicit lifetime
    // Ensures that the returned reference does not outlive the inputs.
    let result = longest(s1, s2);
    println!("The longest string is: {}", result);
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    // Extract the first sentence from the novel.
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    
    // Using struct with lifetime
    // Ensures that 'excerpt' does not outlive 'first_sentence'.
    let excerpt = ImportantExcerpt { part: first_sentence };
    excerpt.announce();
    
    // Using function with lifetime elision
    // Rust infers the lifetime of 'word' based on 's'.
    let word = first_word("Hello world");
    println!("First word: {}", word);
}
