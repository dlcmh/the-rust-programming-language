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
    // which returns a byte slice of a `String`'s contents.
    println!("Bytes of {} are: {:?}", s, s.as_bytes());
    // Bytes of З are: [208, 151]

    let s = "大卫".to_string();

    println!("Length of {} in bytes is {}", s, s.len());
    // Length of 大卫 in bytes is 6

    println!("Bytes of {} are: {:?}", s, s.as_bytes());
    // Bytes of 大卫 are: [229, 164, 167, 229, 141, 171]
}
