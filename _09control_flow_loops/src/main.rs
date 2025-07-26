// Control Flow Loops example in Rust

fn main() {

    // //Loop keyword
    // loop {
    //     println!("This is an infinite loop. Press Ctrl+C to exit.");
    //     // Uncomment the next line to break the loop after one iteration
    //     // break;
    // }


    // While Loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        println!("Counter: {}", counter);
        if counter == 5 {
            break counter * 2; // Breaks the loop and returns the value
        }
    };

    println!("Result of the loop: {}", result);


    // Loop labels

    let mut outer_counter = 0;
    'counting_up: loop {
        println!("count = {}", outer_counter);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9{
                break;
            }
            if outer_counter == 2{
                break 'counting_up;
            }
        remaining -= 1;
        }
        outer_counter += 1;
    }

    println!("End count = {}", outer_counter);


    // While Loop

    let mut number = 3;
    while number != 0{
        println!("{number}");
        number -= 1;
    }
    println!("HEY YA!");

    // For Loop
    let a = [1, 2, 3, 4, 5, 6];

    let b = ["a", "b", "c", "d", "e"];

    for element in a{
        println!("{}", element)
    }

    for letter in b{
        println!("{}", letter)
    }
}
