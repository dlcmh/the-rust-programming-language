struct Point<T> {
    x: T,
    y: T,
}

// Implement methods on structs & enums, and use generic types in their definitions.
// `x` is a method implemented on the `Point<T>` struct.
// The `x` method returns a reference to the data in field `x`.
//
// (A) - unconstrained generic type `T`
// - methods defined within such an `impl` will be defined on __any__ instance of the type,
//   regardless of what concrete type ends up substituting for the generic type. In contrast,
//   see (B).
// For the `<T>` declaration, any other letter could have been used, eg `<U>`, etc.
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
//
// (B) - define methods on the `Point<T>` type with some constraint on the generic type,
//   eg by implementing methods only on `Point<f32>` instances.
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    // (A) works with `i32` values
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
    // p.x = 5

    // (A) works with `f64` values
    let p = Point { x: 5.0, y: 10.0 };
    println!("p.x = {}", p.x());
    // p.x = 5

    // (A) works with `u8` values
    let p = Point { x: 5u8, y: 10u8 };
    println!("p.x = {}", p.x());
    // p.x = 5

    // (A) works with `u16` values
    let p = Point {
        x: 5_u16,
        y: 10_u16,
    };
    println!("p.x = {}", p.x());
    // p.x = 5

    let p = Point { x: 3_f32, y: 4_f32 };
    println!("distance_from_origin: {}", p.distance_from_origin());
    // distance_from_origin: 5
}
