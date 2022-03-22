struct Point<T> {
    x: T,
    y: T,
}

// Implement methods on structs & enums, and use generic types in their definitions.
// `x` is a method implemented on the `Point<T>` struct.
// The `x` method returns a reference to the data in field `x`.
//
//
// For the `<T>` declaration, any other letter could have been used, eg `<U>`, etc.
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    // (A) works with `i32` values
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
    // p.x = 5

    // (B) works with `f64` values
    let p = Point { x: 5.0, y: 10.0 };
    println!("p.x = {}", p.x());
    // p.x = 5

    // (C) works with `u8` values
    let p = Point { x: 5u8, y: 10u8 };
    println!("p.x = {}", p.x());
    // p.x = 5

    // (D) works with `u16` values
    let p = Point {
        x: 5_u16,
        y: 10_u16,
    };
    println!("p.x = {}", p.x());
    // p.x = 5
}
