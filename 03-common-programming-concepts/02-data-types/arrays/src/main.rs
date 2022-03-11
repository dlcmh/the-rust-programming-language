fn main() {
    // not allowed - all elements must have the same type
    // let a = [1, 2, 3, 4, 's'];
    // mismatched types
    // expected integer, found `char`rustcE0308

    let a = [1, 2, 3, 4, 5];
    // println!("a: {}", a);
    //     error[E0277]: `[{integer}; 5]` doesn't implement `std::fmt::Display`
    //     --> src/main.rs:9:23
    //      |
    //    9 |     println!("a: {}", a);
    //      |                       ^ `[{integer}; 5]` cannot be formatted with the default formatter
    //      |
    //      = help: the trait `std::fmt::Display` is not implemented for `[{integer}; 5]`
    //      = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead

    println!("a: {:?}", a); // a: [1, 2, 3, 4, 5]

    println!("a: {:#?}", a);
    // a: [
    //     1,
    //     2,
    //     3,
    //     4,
    //     5,
    // ]
}
