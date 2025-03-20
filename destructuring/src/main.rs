/*
    Destructuring: A technique that allows you to break down complex data structures like tuples, structs,
                   and enums into individual components. This makes it easy to extract and work with specific
                   values without manually indexing or accessing fields.
*/

fn main() {
    // 1. Destructuring a tuple
    let my_tuple = (10, "hello", 3.14);

    // Destructure using a 'let' statement
    let (a, b, c) = my_tuple;
    println!("Tuple values: a = {}, b = {}, c = {}", a, b, c);

    // Partial destructuring
    let (x, _, z) = my_tuple; // Using '_' to ignore values
    println!("Partial tuple: x = {}, z = {}", x, z);

    // 2. Destructuring arrays and slices
    let my_array = [1, 2, 3, 4, 5];

    // Using patterns to destructure
    let [first, second, ..] = my_array; // '..' ignores the rest
    println!("Array values: first = {}, second = {}", first, second);

    // 3. Destructuring structs
    struct Person {
        name: String,
        age: u8,
    }

    let person = Person {
        name: String::from("Alice"),
        age: 25,
    };

    // Destructuring a struct
    let Person { ref name, age } = person;
    println!("Person's name is {} and they are {} years old", name, age);

    // Partial destructuring with ignore
    let Person { ref name, .. } = person;
    println!("Person's name is {}", name);

    // 4. Destructuring enums
    enum Shape {
        Circle { radius: f32 },
        Rectangle { width: f32, height: f32 },
    }

    let shape = Shape::Circle { radius: 5.0 };
    let _shape1 = Shape::Rectangle {
        width: 10.0,
        height: 20.0,
    };

    // Using 'match' for destructuring
    match shape {
        Shape::Circle { radius } => {
            println!("It's a circle with radius {}", radius);
        }
        Shape::Rectangle { width, height } => {
            println!("It's a rectangle with width {} and height {}", width, height);
        }
    }

    // 5. Destructuring in function parameters
    fn print_point((x, y): (i32, i32)) {
        println!("Point is at ({}, {})", x, y);
    }

    let point = (3, 4);
    print_point(point);

    // 6. Destructuring with 'if let'
    let some_value = Some(42);
    if let Some(value) = some_value {
        println!("Got a value: {}", value);
    }

    // 7. Nested destructuring
    let nested_tuple = ((1, 2), 3);
    let ((x, y), z) = nested_tuple;
    println!("Nested tuple values: x = {}, y = {}, z = {}", x, y, z);
}
