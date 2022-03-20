fn text_a(text: &str) -> Option<char> {
    // chars:
    // Returns an iterator over the [char]s of a string slice.
    // As a string slice consists of valid UTF-8, we can iterate through a string slice by [char]. This method returns such an iterator.
    // It's important to remember that [char] represents a Unicode Scalar Value, and might not match your idea of what a 'character' is. Iteration over grapheme clusters may be what you actually want. This functionality is not provided by Rust's standard library, check crates.io instead.
    // won't work with Hindi strings such as नमस्ते, the last of which, for eg, has a diacritic
    text.lines().next()?.chars().last()
}

fn main() {
    // println!("{}", text_a("Hello\n and the World!").unwrap()); // o

    println!("{:?}", text_a("\n and the World!"));
    // thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/main.rs:12:48
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
}
