// `Option<T>` is an enum that is generic over type `T` and has twi variants:
// - `Some`, which holds one value of type `T`, and
// - a `None` variant that doesn't hold any value
// This enum expresses the concept of having an optional value, no matter the type of value.
enum Option<T> {
    Some(T),
    None,
}

// `Result<T, E>` is an enum that uses multiple generic types, in other words,
// it is generic over two types, `T` and `E`.
// Variant `Ok` hold a value of type `T`, while `Err` holds a value of type `E`.
enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    println!("Hello, world!");
}
