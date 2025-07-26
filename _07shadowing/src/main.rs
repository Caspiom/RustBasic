// Shadowing example in Rust
// Shadowing is not the same as mutability.

fn main() {
    let x = 5;

    let x = x + 1; // Shadowing the previous value of x

    {
        let x = x * 2; // Shadowing again in a new scope
        println!("The value of x in the inner scope is: {}", x); // This will print the value of x in the inner scope
    }

    println!("The value of x in the outer scope is: {}", x); // This will print the value of x after the first shadowing
}

