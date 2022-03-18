use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

fn main() {
    // Define an enumeration that enumerates all possible variants of the 2 major standards used for IP addresses
    #[derive(Debug)]
    enum IpAddrKind {
        V4, // version 4
        V6, // version 6
    }

    // Create instance of a variant
    let v4 = IpAddrKind::V4;

    // define a function that takes any `IpAddrKind`
    fn route(ip_kind: IpAddrKind) {
        println!("got: {:?}", ip_kind);
    }

    // route("v4"); // got: V4
    // error[E0308]: mismatched types
    //   --> src/main.rs:18:11
    //    |
    // 18 |     route("v4"); // got: V4
    //    |           ^^^^ expected enum `IpAddrKind`, found `&str`
    route(v4); // got: V4
    route(IpAddrKind::V6); // got: V6

    // Instead of using a struct, attach data to the enum directly;
    // the new enum definition says both variants will have associated `String` values.
    // The name of each variant also becomes a function that takes a `String` argument,
    // constructs and returns an instance of `IpAddr` type.
    #[derive(Debug)]
    enum IpAddr1 {
        V4(String),
        V6(String),
    }

    let home = IpAddr1::V4(String::from("127.0.0.1"));
    println!("home is {:?}", home); // home is V4("127.0.0.1")

    let loopback = IpAddr1::V6(String::from("::1"));
    println!("loopback is {:?}", loopback); // loopback is V6("::1")

    // Unlike structs, each enum variant can have different types and amounts of associated data
    #[derive(Debug)]
    enum IpAddr2 {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr2::V4(127, 0, 0, 1);
    println!("home is {:?}", home); // home is V4(127, 0, 0, 1)

    let loopback = IpAddr2::V6(String::from("::1"));
    println!("loopback is {:?}", loopback); // loopback is V6("::1")

    // use the standard library's `IpAddr` enum
    let home = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    println!("home is {}", home); // home is 127.0.0.1

    let loopback = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
    println!("loopback is {}", loopback); // loopback is ::1

    // define method on enums using `impl`
    #[derive(Debug)]
    enum Message {
        Quit,                       // variant with no associated data
        Move { x: i32, y: i32 },    // variant with named fields like a struct does
        Write(String),              // variant includes a single string
        ChangeColor(i32, i32, i32), // includes three i32 values
    }
    impl Message {
        fn call(&self) {
            println!("&self is {:?}", &self);

            match &self {
                Message::Quit => {}
                Message::Move { x, y } => {
                    println!("x: {}, y: {}", x, y) // x: 600, y: 800
                }
                Message::Write(_) => {}
                Message::ChangeColor(_, _, _) => {}
            }
        }
    }
    let m = Message::Quit;
    m.call(); // &self is Quit
    let m = Message::Write(String::from("hello"));
    m.call(); // &self is Write("hello")
    let m = Message::Move { x: 600, y: 800 };
    m.call(); // &self is Move { x: 600, y: 800 }
    let m = Message::ChangeColor(1, 2, 3);
    m.call(); // &self is ChangeColor(1, 2, 3)
}
