// Enums

fn main() {

    // Enums are used to define a type that can be one of several variants.
    enum IpAddrKind{
        V4,
        V6,
    }

    // You can create instances of the enum variants
    let four = IpAddrKind:: V4;
    let six = IpAddrKind:: V6;

    // You can also create a function that takes an enum as a parameter
    fn route(ip_kind: IpAddrKind){

    }

    // You can call the function with different enum variants
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);   
    

    // Enums can also have data associated with them
    struct IpAddr{
        kind: IpAddrKind,
        address: String,
    }

    // You can create instances of the struct with enum variants
    let home = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("123.45.67.89")
    };

    let loopback = IpAddr{
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };

    enum IpAddrEnum {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    // You can create instances of the enum with data
    let home_enum = IpAddrEnum::V4(123, 45, 67, 89);
    let loopback_enum = IpAddrEnum::V6(String::from("::1"));

}
