fn main() {
    // not allowed - all elements must have the same type
    // let a = [1, 2, 3, 4, 's'];
    // mismatched types
    // expected integer, found `char` rustc(E0308)

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

    // explicitly write an array's type
    let explicit_a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("explicit_a: {:?}", explicit_a); // explicit_a: [1, 2, 3, 4, 5]

    // repeat a value a specified number of times
    let nines = "nines";

    // let length = 9;
    // attempt to use a non-constant value in a constant rustc(E0435)

    // const length = 9;
    // provide a type for the constant: `length: i32`

    // const length: i32 = 9;
    // constant `length` should have an upper case name

    // const LENGTH: i32 = 9;
    //     error[E0308]: mismatched types
    //     --> src/main.rs:46:34
    //      |
    //      |     let repeated_array = [nines; LENGTH];
    //      |                                  ^^^^^^ expected `usize`, found `i32`

    // https://stackoverflow.com/questions/40259802/why-are-all-indexes-in-rust-of-type-usize
    // Slices only allow you to index them using usize, as do all container types that either pretend to be, or actually are, linear in memory. That's because usize is the correct type with which to index them. Any other type would either not be able to access the full potential range of the container, or would allow for indices that cannot possibly exist.
    const LENGTH: usize = 9;

    let repeated_array = [nines; LENGTH];
    println!("{} {}", LENGTH, nines); // 9 nines
    println!("repeated_array {:?}", repeated_array); // repeated_array ["nines", "nines", "nines", "nines", "nines", "nines", "nines", "nines", "nines"]
}
