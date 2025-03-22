/*
   type keyword: Allows for the creation of type aliases.
*/

// Create a type alias for an unsigned 32-bit integer (u32)
type Age = u32;

// Function that takes an Age as a parameter and prints it
fn print_age(age: Age) {
    println!("Age: {}", age);
}

// Define a type alias for a tuple containing a string and an integer
type UserInfo = (String, u32);

// Function to print user info
fn print_user_info(user: UserInfo) {
    println!("User: {}, Age: {}", user.0, user.1);
}

// Main function to demonstrate usage
fn main() {
    // Using the Age type alias
    let my_age: Age = 25;
    print_age(my_age);

    // Using the UserInfo type alias
    let user: UserInfo = (String::from("Alice"), 30);
    print_user_info(user);
}