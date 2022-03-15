fn main() {
    // (A) create new, empty string which can be loaded with data
    let s = String::new();
    println!("s is !{}!", s); // s is !!

    // (B)
    // Start a string with some initial data with the `to_string` method, which
    // is available on any type that implements the `Display` trait, as string
    // literals do. `to_string` method creates a `String` from a string literal.
    let data = "some text";
    let s = data.to_string();
    println!("string literal data is !{}!", data); // string literal data is !some text!
    println!("s is !{}!", s); // s is !some text!

    let s = "my name".to_string();
    println!("s is !{}!", s); // s is !my name!

    // (C) use `String::from` function to create a `String` from a string literal
    let s = String::from("Foundation says: 你好");
    println!("!{}!", s); // !Foundation says: 你好!
}
