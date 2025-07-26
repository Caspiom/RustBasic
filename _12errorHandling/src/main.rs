// Error Handling

    // Aproach 1
    // enum Option<T>{ // Define the generic Option type
    //     Some(T), // Represents a value
    //     None, // Represents no value
    // }

    // // Aproach 2
    // enum Result<T, E>{ // Define the generic Result type
    //     Ok(T), // Representes a Value
    //     Err(E), // Representes an error
    // }

    // Example of using Option in Rust
    // Option is used when you want to return a value or nothing
    fn divide_Opt(numerator: f64, denominator: f64) -> Option<f64>{
        if denominator == 0.0{
            None
        } else {
            Some(numerator / denominator)
        }
    }

    // Example of using Result in Rust
    // Result is used when you want to return a value or an error
    fn divide_Rslt(numerator: f64, denominator: f64) -> Result<f64, String>{
        if denominator == 0.0{
            Err("Cannot divide by 0".to_string())
        } else {
            Ok(numerator / denominator)
        }
    }

fn main() {

    // Option example
    let option = divide_Opt(10.0, 0.0);

    match option{
        Some(x) => println!("Result: {}", x),
        None => println!("Error can't divide by 0"),
    }

    //Result Eample

    match divide_Rslt(100.23, 00.0){
    Ok(result) => println!("Result: {}", result),
    Err(err) => println!("Error{}", err),
    }

}
