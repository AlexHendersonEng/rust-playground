/*
    Enumerations: A way to define a type that can have multiple possible variants.
                  Enums allow you to group related values under a single type, making
                  code more readable and type-safe. Unlike simple integer enums in some
                  other languages, Rust enums can hold data, making them powerful and flexible.
                  They are often used in pattern matching to handle different cases efficiently.
*/

// 1. Basic Enum
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn use_direction() {
    let dir = Direction::Up;
    match dir {
        Direction::Up => println!("Going up!"),
        Direction::Down => println!("Going down!"),
        Direction::Left => println!("Going left!"),
        Direction::Right => println!("Going right!"),
    }
}

// 2. Enum with Tuple Variants
enum Color {
    Red(u8, u8, u8), // RGB values
    Grayscale(u8),   // Single grayscale value
}

fn use_color() {
    let c = Color::Red(255, 0, 0);
    match c {
        Color::Red(r, g, b) => println!("Red: {}, Green: {}, Blue: {}", r, g, b),
        Color::Grayscale(v) => println!("Grayscale: {}", v),
    }
}

// 3. Enum with Struct Variants
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}

fn use_shape() {
    let shape = Shape::Circle { radius: 10.0 };
    match shape {
        Shape::Circle { radius } => println!("Circle with radius: {}", radius),
        Shape::Rectangle { width, height } => println!("Rectangle: {}x{}", width, height),
    }
}

// 4. Enum with Explicit Integer Values
enum StatusCode {
    Success = 200,
    NotFound = 404,
    InternalError = 500,
}

fn use_status_code() {
    let code = StatusCode::NotFound as u16;
    println!("HTTP Status Code: {}", code);
}

// 5. Enum with Methods
enum Animal {
    Dog,
    Cat,
}

impl Animal {
    fn speak(&self) {
        match self {
            Animal::Dog => println!("Woof!"),
            Animal::Cat => println!("Meow!"),
        }
    }
}

fn use_animal() {
    let pet = Animal::Dog;
    pet.speak();
}

// Main function to call all examples
fn main() {
    use_direction();
    use_color();
    use_shape();
    use_status_code();
    use_animal();
}

