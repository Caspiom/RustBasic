fn main() {

    let name = "Gaspari";
    let age: u32 = 25;
    let social_id = "123-456-789";
    let height: u32 = 184;

    let Gaspari = human_id(name, age, social_id);

    let _X: u32 = {
        let price: u32 = 100;
        let qty: u32 = 10;
        price * qty
    };

    hello_world();
    tell_height(height);
    println!("{}", Gaspari);
    println!("Result: {}", _X);
    let y = add(5, 10);
    println!("Y: {}", y);
    println!("Sum: {}", add(3, 4));

    let weight_kg: f32 = 70.0;
    let height_m: f32 = 1.75;
    let bmi = calculate_bmi(weight_kg, height_m);
    println!("BMI: {:.2}", bmi);

}

//functions
//Functions are defined using the `fn` keyword, followed by the function name and parameters in parentheses.
//The function body is enclosed in curly braces `{}`.
fn hello_world(){
    println!("Hello, world!");
}

fn tell_height(height: u32) {
    println!("Height: {} cm", height);
}

//expression
//An expression is a piece of code that evaluates to a value.
//In Rust, expressions can be used in various contexts, such as in function bodies or as part of control flow statements.
fn add(a: i32, b: i32) -> i32 {
    a + b // This is an expression that evaluates to the sum of a and b
}

//this is a example of a expression that returns a formatted string
fn human_id(name: &str, age: u32, social_id: &str) -> String {
    format!("Name: {}, Age: {}, Social ID: {}", name, age, social_id)
}

fn calculate_bmi(weight_kg: f32, height_m: f32) -> f32 {
    weight_kg/ (height_m * height_m)
}