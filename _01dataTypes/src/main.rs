// compound data types
// arrays, tuples, slices, and strings (slice str)

// This code demonstrates the use of arrays, tuples, slices, and strings in Rust.

fn main() {

    //arrays
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array of numbers: {:?}", numbers);

    let fruits: [&str; 3] = ["apple", "banana", "cherry"];
    println!("Array of fruits first element: {}", fruits[0]);
    println!("Array of fruis second element: {}", fruits[1]);
    println!("Array of fruits third element: {}", fruits[2]);

    //tuples
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human tuple: {:?}", human);

    let my_mix_tuple = ("Kratos", 23, true, [1, 2, 3, 4, 5]);
    println!("My mix tuple: {:?}", my_mix_tuple);

    //slices (alocados em sequencia na memória)
    let numbers_slice:&[i32] = &[1, 2, 3, 4, 5];
    println!("Slice of numbers: {:?}", numbers_slice);

    let animals: &[&str] = &["dog", "cat", "bird"];
    println!("Slice of animals: {:?}", animals);

    let book_slice: &[&String] = &[&"The Hobbit".to_string(), &"1984".to_string(), &"To Kill a Mockingbird".to_string()];
    println!("Slice of books: {:?}", book_slice);

    //Strings Vs String Slices(&str)

    //String é uma estrutura de dados alocada no heap e que pode crescer
    let mut pikachu_say: String = String::from("Pika");
    println!("Pikachu says: {}", pikachu_say);
    pikachu_say.push_str(" chu!");
    println!("Pikachu now says: {}", pikachu_say);

    //&str é uma referência imutável para uma fatia de string
    let string: String = String::from("Hello, Rust!");
    let string_slice: &str = &string[0..5]; // "Hello"
    println!("String slice: {}", string_slice);
 }

// This code demonstrates the use of arrays, tuples, slices, and strings in Rust.