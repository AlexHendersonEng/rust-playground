/*
    Match guards: Additional conditions that can be added to a pattern in a 'match'
                  expression. These guards are written after a pattern and a 'if' keyword.
                  They are useful when you want to refine the conditions under which a 
                  particular pattern is matched.
*/

fn main() {
    let number = 5;

    // Using match with a guard
    match number {
        n if n < 0 => println!("{} is a negative number.", n),
        n if n == 0 => println!("{} is zero.", n),
        n if n > 0 && n <= 10 => println!("{} is a positive number between 1 and 10.", n),
        _ => println!("{} is greater than 10.", number),
    }

    // Another example using match guards with enums
    enum Shape {
        Circle(f64),
        Square(f64),
    }

    let shape = Shape::Circle(5.0);
    let _shape1 = Shape::Square(10.0);

    match shape {
        Shape::Circle(radius) if radius < 10.0 => {
            println!("This is a small circle with radius {}.", radius);
        }
        Shape::Circle(radius) => {
            println!("This is a large circle with radius {}.", radius);
        }
        Shape::Square(side) if side < 5.0 => {
            println!("This is a small square with side length {}.", side);
        }
        Shape::Square(side) => {
            println!("This is a large square with side length {}.", side);
        }
    }
} 
