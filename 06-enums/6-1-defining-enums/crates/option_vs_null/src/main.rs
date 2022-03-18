fn main() {
    let some_number = Some(5);
    println!("some_number: {:?}", some_number); // some_number: Some(5)

    let some_str: Option<&str> = Some("hello");
    println!("some_str: {:?}", some_str); // some_str: Some("hello")

    // type of some_string is inferred as `Option<string>`
    let some_string = Some("bye".to_string());
    println!("some_string: {:?}", some_string); // some_string: Some("bye")

    let empty_number: Option<i32> = None;
    println!("empty_number: {:?}", empty_number); // empty_number: None

    // compiler prevents use of an `Option<T>` value as if it were a valid value
    // this lets us proceed confidently without having to check for null before
    //   using that value
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y;
    // --> src/main.rs:21:17
    // |
    // 21 |     let sum = x + y;
    // |                 ^ no implementation for `i8 + Option<i8>`
    // |
}
