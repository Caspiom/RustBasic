// Collection Types in Rust
// Vectors, HashMaps, and UTF-8 Strings
use std::collections::HashMap;

fn main() {

    // ======================================== Vectors ====================================================
    // Vectors are resizable arrays that can grow or shrink in size.
    // They are stored on the heap and can hold elements of the same type.
    // Vectors are defined using the `Vec<T>` type, where `T` is the type of elements in the vector.
    // You can create a new vector using `Vec::new()` or the `vec![]` macro.
    // Vectors can be mutable, allowing you to add or remove elements.
    // You can access elements using indexing, and you can iterate over them using a for loop
    let mut _v: Vec<i32> = Vec::new();

    let mut _v: Vec<i32> = vec![1,2,3,4,5]; // Using the vec! macro to create a vector

    _v.push(6); // Adding elements to the vector
    _v.push(7);
    _v.push(8);
    _v.push(9);
    _v.push(10);
    println!("Vector: {:?}", _v);

    let third_element: &i32 = &_v[2]; // Accessing the third element 

    println!("Third element: {}", third_element);

    let third_element: Option<&i32> = _v.get(2); // Using get method to access the third element safely
    match third_element {
        Some(value) => println!("Third element: {}", value),
        None => println!("No third element found"),
    }

    // ======================================== UTF-8 Strings ====================================================
    // Rust's `String` type is a UTF-8 encoded, growable string type.
    // It is stored on the heap and can hold any valid UTF-8 sequence.
    // You can create a new string using `String::from()` or the `to_string()` method.
    // Strings can be concatenated using the `+` operator or the `push_str` method.
    // You can also iterate over the characters in a string using a for loop

    let mut s: String = "whatever".to_string(); // Creating a new String with to_string()
    let mut s: String = String::from("whatever"); // Creating a new String with String::from()

    s.push_str(" is a string"); // Push_str can add a string slice to the end of the String
    s.push('!'); // Push can add only one character at a time
    println!("String: {}", s);

    let s1: String = String::from("Hello, ");
    let s2: String = String::from("world!");
    let s3: String = s1 + &s2; // Concatenating two strings using s1 as the base and s2 as a reference
    // Note: s1 is moved here, so it cannot be used after this line because ownership is transferred to s3
    println!("Concatenated String: {}", s3);


    // ======================================== HashMaps ====================================================
    // HashMaps are collections of key-value pairs, where each key is unique.
    // They are implemented using a hash table and can store values of any type.
    // You can create a new HashMap using `HashMap::new()` or the `hashmap!` macro.
    // HashMaps can be mutable, allowing you to add, remove, or modify key-value pairs.
    // You can access values using keys, and you can iterate over the key-value pairs using a for loop


    let mut scores = HashMap::new(); // Creating a new HashMap
    scores.insert(String::from("Vitória"), 7);
    scores.insert(String::from("Derrota"), 3);

    let team_name: String = String::from("Vitória");
    let score = scores.get(&team_name).copied().unwrap_or(0); // Accessing the value using a key

    println!("Score for {}: {}", team_name, score);

    for (key, value) in &scores{
        println!("{}: {}", key, value);
    }
}