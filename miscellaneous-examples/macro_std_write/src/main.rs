// https://doc.rust-lang.org/std/macro.write.html
//
// How do I push_str the contents of a variable? - help - The Rust Programming Language Forum
// - https://users.rust-lang.org/t/how-do-i-push-str-the-contents-of-a-variable/45594
// `format!` gives you a `String`, so you need to take a reference to that to get a `&str`.
// However this allocates a new `String` just to throw it away after it copied to `some_string`.
// `write!` directly writes to `some_string`.

// https://doc.rust-lang.org/std/fmt/trait.Write.html
// A trait for writing or formatting into Unicode-accepting buffers or streams.
use std::fmt::Write;

// Need `std::fmt::Result` (`std::io::Result<()>` if `io::Write` trait had been used) & `Ok(())`,
// otherwise have to use `write!(..).unwrap()` instead of `write!(..)?`.
fn main() -> std::fmt::Result {
    // (A) type of s1 is &str
    let s1 = "s1";

    // (B) type of s2 is String
    let mut s2 = format!("{}", "s2");

    // (C) type of s3 is String
    let mut s3 = vec!["one", "two", "three"].join(", ");

    // (D) push &str onto String
    s2.push_str(s1);

    // (E) push &String onto String
    s3.push_str(&s2);

    // (F) push String onto String with &format!
    let mut s4 = String::new();
    s4.push_str(&format!("{}", s2));

    // (G) push &str onto String with &format!
    let mut s5 = String::new();
    s5.push_str(&format!("{}", s1));

    // (H) append directly to String
    write!(&mut s5, "&str: {}, String: {}", s1, s2)?;

    println!("s1: {}", s1);
    println!("s2: {}", s2);
    println!("s3: {}", s3);
    println!("s4: {}", s4);
    println!("s5: {}", s5);

    // Results:
    // s1: s1
    // s2: s2s1
    // s3: one, two, threes2s1
    // s4: s2s1
    // s5: s1&str: s1, String: s2s1

    Ok(())
}
