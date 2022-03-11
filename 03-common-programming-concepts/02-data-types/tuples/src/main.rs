fn main() {
    let tup = (500, 6.4, '😅', "Rust");

    // you may be able to use `{:?}` (or {:#?} for pretty-print) instead
    println!("tup: {:?}", tup); // tup: (500, 6.4, '😅', "Rust")

    println!("tup: {:#?}", tup);
    // tup: (
    //     500,
    //     6.4,
    //     '😅',
    //     "Rust",
    // )

    // use pattern matching to destructure a tuple value
    // unused variables - if this is intentional, prefix it with underscore
    let (_w, _x, y, _z) = tup;
    println!("grinning face with sweat: {}", y); // grinning face with sweat: 😅

    // directly access an element with a period . and desired index
    let grinning_face_with_sweat = tup.2;
    println!("an emoji: {}", grinning_face_with_sweat); // an emoji: 😅

    // the type of a tuple without any values, (), is called the unit type, which is a special type that has only one value, known as unit value, also written as ()
    let unit: () = ();

    // `()` doesn't implement `std::fmt::Display`
    // the trait `std::fmt::Display` is not implemented for `()`
    // in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) insteadrustcE0277
    // println!("unit value: {}", unit);
    println!("unit value: {:?}", unit); // unit value: ()
    println!("unit value: {:#?}", unit); // unit value: ()
}
