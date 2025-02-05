/*
    if let syntax: Lets you combine if and let into a less verbose way to handle values
                   that match one pattern while ignoring the rest.
*/

fn main() {
    let some_number = Some(42);

    if let Some(value) = some_number {
        println!("The number is: {}", value);
    }
}
