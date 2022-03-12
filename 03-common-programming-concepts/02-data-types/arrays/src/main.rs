use std::io;

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

    // use indexing to access an array's element
    println!("a[0]: {}", a[0]); // a[0]: 1
    println!("a[4]: {}", a[4]); // a[4]: 5

    // explicitly write an array's type
    let explicit_a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("explicit_a: {:?}", explicit_a); // explicit_a: [1, 2, 3, 4, 5]

    // repeat a value a specified number of times
    let nines = "nines";

    // let length = 9;
    //     error[E0435]: attempt to use a non-constant value in a constant
    //     --> src/main.rs:57:34
    //      |
    //      |     let length = 9;
    //      |     ---------- help: consider using `const` instead of `let`: `const length`

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

    // invalid access of an element that is past the end of the array results in a runtime error - Rust will panic
    // Rust's error handling will be discussed in Chapter 9
    loop {
        println!("Please enter an array index.");
        let mut index = String::new();
        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line.");
        let index: usize = index
            .trim()
            .parse()
            .expect("Index entered was not a number.");
        println!("Element at index {} is: {}", index, a[index]);
        // Please enter an array index.
        // 2
        // Element at index 2 is: 3
        // Please enter an array index.
        // 1
        // Element at index 1 is: 2
        // Please enter an array index.
        // 5
        // thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 5', src/main.rs:82:55
        // stack backtrace:
        //    0: rust_begin_unwind
        //              at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/std/src/panicking.rs:498:5
        //    1: core::panicking::panic_fmt
        //              at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/panicking.rs:116:14
        //    2: core::panicking::panic_bounds_check
        //              at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/panicking.rs:84:5
        //    3: arrays::main
        //              at ./src/main.rs:82:55
        //    4: core::ops::function::FnOnce::call_once
        //              at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/ops/function.rs:227:5
        // note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
        // The terminal process "cargo 'run', '--package', 'arrays', '--bin', 'arrays'" terminated with exit code: 101.
    }
}
