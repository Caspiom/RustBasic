// Rule 1 - Each variable has a single owner
// Rule 2 - There can only be one owner at a time
// Rule 3 - When the owner goes out of scope, the value will be dropped
// Ownership is a key concept in Rust that ensures memory safety without needing a garbage collector.
// In this example, we will demonstrate ownership and borrowing in Rust.
// The main function is the entry point of the program.
// It initializes some variables, calls functions, and prints results.
// The ownership of the variable `s1` is transferred to the function `calculate_length`
// using a reference, which allows us to access the value without taking ownership.
// The function `calculate_length` takes a reference to a `String` and returns its length
// without consuming the original `String`.
// The main function is the entry point of the program. 
fn main() {
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);
    println!("Length of '{}' is {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
