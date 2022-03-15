fn main() {
    let mut s = String::from("abc");
    println!("!{}!", s); // !abc!

    // (A)
    // grow a String with the `push_str` method - appends a string slice to a String
    s.push_str("def"); // "def" is a string literal of type &str, a borrowed string slice
    println!("!{}!", s); // !abcdef!

    // (B) - `push` appends a single character
    s.push('😻');
    println!("!{}!", s); // !abcdef😻!

    // (C)
    // use the `+` operator to concatenate two `String`s.
    // `+`'s signature is `fn add(self, s: &str) -> {`, meaning two `String`
    // values cannot be added together - we can only add a `&str` to a `String`.
    // If needed, the compiler can coerce a `&String` into a `&str`, allowing us
    // to add a `&String` to a `String`. This coercion is known as "deref coercion",
    // which turns a borrowed string slice into a String collection type - assuming
    // `let s3 = s1 + &s2`, it means `&s2` is coerced into `&s2[..]`. The first `String`,
    // represented by `self` is taken actual ownership of by `add` (because self does not have an `&`),
    // to which `add` appends a copy of `s` and returns the ownership of the result.

    // s += String::from(" - ");
    // error[E0308]: mismatched types
    //   --> src/main.rs:26:10
    //    |
    // 25 |     s += String::from(" - ");
    //    |          ^^^^^^^^^^^^^^^^^^^
    //    |          |
    //    |          expected `&str`, found struct `String`
    //    |          help: consider borrowing here: `&String::from(" - ")`

    s += &String::from(" - ");
    s += &String::from("Hi there!");
    println!("!{}!", s); // !abcdef😻 - Hi there!!

    // (D)
    let s = String::from("The") + &String::from("The") + &String::from("The") + "The";
    println!("!{}!", s); // !TheTheTheThe!

    // with the `format!` macro - uses references so that
    // no ownership is taken of its parameters
    let s = format!(
        "{}{}{}{}",
        String::from("The"),
        String::from("The"),
        String::from("The"),
        "The"
    );
    println!("!{}!", s); // !TheTheTheThe!
}
