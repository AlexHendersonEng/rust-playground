/*
    Generics: Used to create definitions for items like function signatures
              or structs, which we can then use with many different concrete
              data types.
*/

// A generic function that works with any type T
fn generic_function<T>(x: T) {
    println!("This is a generic function!");
}

// A generic struct that holds any type T
struct GenericStruct<T> {
    value: T,
}

// A generic implementation for the struct
impl<T> GenericStruct<T> {
    fn new(value: T) -> Self {
        Self { value }
    }

    fn get_value(&self) -> &T {
        &self.value
    }
}

// A generic enum that can hold different types
enum GenericEnum<T> {
    SomeValue(T),
    NoneValue,
}

fn main() {
    // Using the generic function
    generic_function(42);
    generic_function("Hello");
    
    // Using the generic struct
    let int_struct = GenericStruct::new(10);
    let str_struct = GenericStruct::new("Rust");
    println!("Value in int_struct: {}", int_struct.get_value());
    println!("Value in str_struct: {}", str_struct.get_value());
    
    // Using the generic enum
    let some_number: GenericEnum<i32> = GenericEnum::SomeValue(100);
    let some_text: GenericEnum<&str> = GenericEnum::SomeValue("Generic Enum");
    let none_value: GenericEnum<i32> = GenericEnum::NoneValue;
}
