fn main() {
    // A `String` is a wrapper over a `Vec<u8>`.
    // If the length of a `String` is 4, it means the vector storing
    // the string, assumed to be encoded in UTF-8, is 4 bytes long. It CANNOT be
    // assumed the string itself is exactly 4 characters in length. A character,
    // represented by a Unicode scalar value, takes up at least one byte of
    // storage, possibly more, depending on the encoding requirements of UTF-8.

    let s = "David".to_string();
    println!("Length of {} in bytes is {}", s, s.len());
    // Length of David in bytes is 5

    let s = "~".to_string();
    println!("Length of {} in bytes is {}", s, s.len());
    // Length of ~ in bytes is 1

    println!("Bytes of {} are: {:?}", s, s.as_bytes());
    // Bytes of ~ are: [126]

    // З is the capital Cyrillic letter Ze, not the Arabic number 3
    let s = "З".to_string();
    // vscode highlights З and says:
    // The character U+0417 "З" could be confused with the character U+0033 "3", which is more common in source code.

    println!("Length of {} in bytes is {}", s, s.len());
    // Length of З in bytes is 2

    // When encoded in UTF-8, the first byte of З is 208 and the second is 151.

    // in Ruby `irb`:
    // "З".each_byte.to_a: # => [208, 151]
    // "大".each_byte.to_a # => [229, 164, 167]

    // In Rust, using `as_bytes` method (pub fn as_bytes(&self) -> &[u8]),
    // which returns a byte slice of:
    // - a `String`'s contents, or
    // - a string slice's contents
    println!("Bytes of {} are: {:?}", s, s.as_bytes());
    // Bytes of З are: [208, 151]

    let s = "大卫".to_string();

    println!("Length of {} in bytes is {}", s, s.len());
    // Length of 大卫 in bytes is 6

    println!("Bytes of {} are: {:?}", s, s.as_bytes());
    // Bytes of 大卫 are: [229, 164, 167, 229, 141, 171]

    // slicing a `String`, `s`, using `[]` with a range
    // println!("{}", s[0..3]);
    // error[E0277]: the size for values of type `str` cannot be known at compilation time
    //    --> src/main.rs:50:5
    //     |
    // 50  |     println!("{}", s[0..3]);
    //     |     ^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
    //     |
    //     = help: the trait `Sized` is not implemented for `str`
    //
    // Read [Why your first FizzBuzz implementation may not work - an article by Chris Morgan](https://chrismorgan.info/blog/rust-fizzbuzz/) on why Rust has 2 types of strings, unlike C#, Java, & Go (Golang) which have only 1.
    //
    // Rust’s model is somewhat different, not being a garbage collected language, being centred around its language-wide model of ownership, where each object is owned in one place at a time, though other places may safely borrow references to it.
    //
    // `String` is an owned type. That is, it has exclusive ownership of the contents of the string; and when it passes out of scope, the memory for the contents of the string will be freed immediately. For this reason, any substring can’t be of the type `String`, for there will be no connection between the two, and so when one passed out of scope, the other would become invalid, leading to a loss of memory safety. And so instead it is that slices (substrings) use a type which is a reference to the contents that something else owns - `​&str`. Rust, through its concept of lifetimes, is able to guarantee that no slice outlives the actual String, and so memory safety is ensured.
    //

    // slicing a `String`, `&s`, using `[]` with a range
    println!("Sliced with [0..3]: {}", &s[0..3]);
    // Sliced with [0..3]: 大

    // Rust will panic if an entire character isn't indexed in a valid fashion
    println!("Sliced with [0..1]: {}", &s[0..1]);
    // thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside '大' (bytes 0..3) of `大卫`', src/main.rs:63:41
}
