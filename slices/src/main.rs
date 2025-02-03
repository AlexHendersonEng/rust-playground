/*
    Slices: A reference to a contiguous sequence of elements within a collection,
            like an array or a String. It allows borrowing a subset of data without
            taking ownership, using a range (&arr[start..end]). Slices are useful
            for efficiently working with parts of data without copying or moving them.
*/

fn main() {
    // Basic array slicing
    let arr = [1, 2, 3, 4, 5];
    let slice1: &[i32] = &arr[1..4]; // Slicing from index 1 to 3
    println!("Slice of array: {:?}", slice1);

    // String slicing
    let s = String::from("Hello, world!");
    let slice2: &str = &s[0..5]; // Slicing the first 5 characters
    println!("Slice of string: {}", slice2);

    // Mutable slice example
    let mut nums = [10, 20, 30, 40, 50];
    let slice3: &mut [i32] = &mut nums[2..4]; // Mutating part of the array
    slice3[0] = 99;
    println!("Mutated slice: {:?}", nums);

    // Iterating over a slice
    let slice4 = &arr[..]; // Full slice
    for &num in slice4 {
        print!("{} ", num);
    }
    println!();

    // Function that takes a slice
    print_slice(&arr[1..4]);

    // Converting a vector to a slice
    let vec = vec![100, 200, 300];
    let slice5: &[i32] = &vec[..2]; // Slicing first two elements
    println!("Slice from vector: {:?}", slice5);
}

// Function with slice as a parameter
fn print_slice(slice: &[i32]) {
    println!("Printing slice: {:?}", slice);
}

