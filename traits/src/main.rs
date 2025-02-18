/*
    Traits: Defines the functionality a particular type has and can share with
            other types. Traits can be used to define shared behavior in an
            abstract way. Trait bounds can be used to specify that a generic
            type can be any type that has certain behavior.
*/

// Define a basic trait
trait Greet {
    fn greet(&self); // A method that must be implemented by types using this trait
}

// Implement the trait for a struct representing a person
struct Person;

impl Greet for Person {
    fn greet(&self) {
        println!("Hello from Person!"); // Implementation of the greet method for Person
    }
}

// Implement the trait for another struct representing an animal
struct Animal;

impl Greet for Animal {
    fn greet(&self) {
        println!("Animal makes a sound!"); // Implementation of the greet method for Animal
    }
}

// Using trait objects (dynamic dispatch)
// This allows multiple types implementing Greet to be handled via a single function
fn say_hello_dynamic(greeter: &dyn Greet) {
    greeter.greet();
}

// Using generic constraints (static dispatch)
// This is resolved at compile-time, leading to potentially better performance
fn say_hello_generic<T: Greet>(greeter: &T) {
    greeter.greet();
}

// Using trait bounds with a where clause (alternative way of defining constraints)
fn say_hello_with_bound<T>(greeter: &T) where T: Greet {
    greeter.greet();
}

fn main() {
    let person = Person;
    let animal = Animal;
    
    // Calling methods directly
    person.greet(); // Direct call to the greet method on a Person instance
    animal.greet(); // Direct call to the greet method on an Animal instance
    
    // Using trait object (dynamic dispatch)
    say_hello_dynamic(&person); // Calls greet via a trait object
    say_hello_dynamic(&animal);
    
    // Using generics (static dispatch)
    say_hello_generic(&person); // Calls greet via generic constraints
    say_hello_generic(&animal);
    
    // Using trait bounds
    say_hello_with_bound(&person); // Calls greet via trait bounds
    say_hello_with_bound(&animal);
}
