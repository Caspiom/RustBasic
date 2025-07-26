// If Else [control flow example in Rust]

use std::io;

fn main() {

    let mut gappy_age: String = String::new();
    
    println!("Enter Gappy's age:");
    io::stdin().read_line(&mut gappy_age).expect("Failed to read line");
    let gappy_age: u32 = gappy_age.trim().parse().expect("Please enter a valid number");

    if gappy_age < 18 {
        println!("Gappy can't drive yet.");
    } else {
        println!("Gappy can drive !");
    }

    let mut gappy_height: String = String::new();
    println!("Enter Gappy's height in meters:");
    io::stdin().read_line(&mut gappy_height).expect("Failed to read line");
    let gappy_height: f32 = gappy_height.trim().parse().expect("Please enter a valid number");


    if gappy_height >= 1.80 {
        println!("Gappy is a tall person.");
    } else if gappy_height < 1.60 {
        println!("Gappy is a short person.");
    } else {
        println!("Gappy is of average height.");
    }

    let condition = true;
    let number = if condition { 5.0 } else { 6.0 };
    println!("The value of number is: {}", number);

}
