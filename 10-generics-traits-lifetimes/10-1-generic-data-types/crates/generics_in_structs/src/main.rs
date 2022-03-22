// `Point<T>` is a struct that is generic over some type `T`, and the fields
// `x` and `y` are both that same type.
//
// (A) shows what happens if an instance is created with different types for the
// fields.
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// (B) use multiple generic type parame
#[derive(Debug)]
struct Coordinates<T, U> {
    x: T,
    y: U,
}

fn main() {
    let integer_point = Point { x: 5, y: 10 };
    println!("integer_point: {:?}", integer_point);
    // integer_point: Point { x: 5, y: 10 }

    let float_point = Point { x: 5.0, y: 10.0 };
    println!("float_point: {:?}", float_point);

    // (A)
    // let mixed_point = Point { x: 5, y: 10.0 };
    // println!("mixed_point: {:?}", mixed_point);
    //     error[E0308]: mismatched types
    //     --> src/main.rs:28:40
    //      |
    //   28 |     let mixed_point = Point { x: 5, y: 10.0 };
    //      |                                        ^^^^ expected integer, found floating-point number

    let mixed_coordinates = Coordinates { x: 5, y: 10.0 };
    println!("mixed_coordinates: {:?}", mixed_coordinates);
    // mixed_coordinates: Coordinates { x: 5, y: 10.0 }
}
