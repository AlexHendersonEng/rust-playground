/*
    Strucutures: Allow you to create custom data types that group related data together.
                 They are similar to classes in other languages but do not support inheritance.
                 
                 There are three main types of structs: 
                 1. Named-field struct: Has explicitly named fields.
                 2. Tuple struct: Similar to tuples but with named struct types.
                 3. Unit-like struct: Empty struct useful for implementing traits.
*/

// 1. Defining a basic struct
struct BasicStruct {
    x: i32,
    y: i32,
}

// 2. Tuple struct (like a struct but without named fields)
struct TupleStruct(i32, i32);

// 3. Unit-like struct (useful for implementing traits without fields)
struct UnitStruct;

// 4. Struct with methods (impl block)
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // Method to create a new instance
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    
    // Method to calculate distance from origin
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    // Using a basic struct
    let basic = BasicStruct { x: 10, y: 20 };
    println!("BasicStruct: x = {}, y = {}", basic.x, basic.y);

    // Using a tuple struct
    let tuple = TupleStruct(5, 15);
    println!("TupleStruct: 0 = {}, 1 = {}", tuple.0, tuple.1);

    // Using a unit-like struct (no fields, so not much to do with it)
    let _unit = UnitStruct; // Just an instance

    // Using a struct with methods
    let point = Point::new(3.0, 4.0);
    println!("Point: x = {}, y = {}", point.x, point.y);
    println!("Distance from origin: {}", point.distance_from_origin());
}