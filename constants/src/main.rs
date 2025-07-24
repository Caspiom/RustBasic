//Constants in Rust

fn main() {
    //let mut x = 5;
    //const mut y = 10;
    //println!("x: {}, y: {}", x, y); // This line will cause a compile-time error because y is a constant and cannot be mutable

    let mut x = 5;
    const Y: i32 = 10; // Constants are immutable by default
    println!("x: {}, Y: {}", x, Y);
    println!("The value of PI is: {}", PI); // Accessing the constant PI defined below
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);
    println!("Max points: {}", MAX_POINTS);
}

// You can declare a constant with a type here
const PI: f64 = 3.14159;
const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60; // Constants can be defined with expressions
const MAX_POINTS: u32 = 100_000; // Constants can also use underscores for readability

