// Structs
// Stricts are used to name and package related values similar to tuples.

fn main() {
    // Struct
    struct Book{
        title: String,
        author: String,
        pages: u32,
        available: bool,
    }

    struct User{
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut user1: User = User{
        active: true,
        username: String::from("user"),
        email: String::from("user@mail.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("user1@mail.com");

    println!("user email is: {}", user1.email);

    // You can return a struct from a function 
    fn build_user(email: String, username: String) -> User{
        User{
            active: true,
            email,
            username,
            sign_in_count: 0,
        }
    }

    // You can create instances from other instances
    let user2 = User{
        email: String::from("otheremail@mail.com"),
        ..user1
    };

    // Tuple Structs
    struct Color(i32, i32, i32);

    let black: Color = Color(0,0,0);
    let white: Color = Color(255,255,255);

    // Unit-Like struct
    struct AlwaysEqual;
    let subject: AlwaysEqual = AlwaysEqual;
    
}
