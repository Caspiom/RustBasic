fn main() {
    //let _a: u16 = 10;
    //println!("The value of _a is: {}", _a);
    // _a = 15; // This line will cause a compile-time error because _a is immutable

    let mut a: i16 = 10;
    println!("The value of a is: {}", a);
    a = -15; // This line is valid because a is mutable
    println!("The value of a is now: {}", a);
}
