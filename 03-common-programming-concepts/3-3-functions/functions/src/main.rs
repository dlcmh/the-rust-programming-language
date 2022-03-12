fn main() {
    this_function();

    that_function(); // x is 2

    print_labeled_measurement(10, 'h'); // The measurement is: 10h

    let _y = 6; // this is an example of a statement

    // ## Because statements don't return values,
    //   it's not allowed to assign the following `let _x = 6` statement to _w
    //   as there's nothing for _w to bind to.
    //   In C & Ruby, x = y = 6 is allowed, but not in Rust.
    // let _w = (let _x = 6);
    // error: expected expression, found statement (`let`)
    // --> src/main.rs:12:15
    //  |
    //  |     let _w = (let _x = 6);
    //  |               ^^^^^^^^^^
    //  |
    //  = note: variable declaration using `let` is a statement

    // ## A new scope block created with curly brackets
    //   is an example of an expression (because a value is returned).
    let z = {
        let x = 3;

        // be careful not to write `x + 1;`, as the presence of a semicolon
        // turns the scope block expression into a statement,
        // which means the value 4 would not be returned
        x + 1
    };
    println!("z is {}", z); // z is 4

    let p = add_one(10); // call a value-returning function
    print!("p is {}", p); // p is 11
}

fn this_function() {}

// function definition order isn't important
// notice `those_functions` are defined after `that_function`
fn that_function() {
    those_functions(2); // calling a function with a concrete value; the concrete value is called an "argument"
}

// x is a "parameter"
fn those_functions(x: i32) {
    // parameters must be typed, otherwise, `fn those_functions(x) {// ...}` results in:
    //   error: expected one of `:`, `@`, or `|`, found `)`
    //     --> src/main.rs:14:21
    //      |
    //      | fn those_functions(x) {
    //      |                     ^ expected one of `:`, `@`, or `|`
    //      |
    //      = note: anonymous parameters are removed in the 2018 edition (see RFC 1685)
    //   help: if this is a `self` type, give it a parameter name
    //      |
    //      | fn those_functions(self: x) {
    //      |                    +++++
    //   help: if this is a parameter name, give it a type
    //      |
    //      | fn those_functions(x: TypeName) {
    //      |                     ++++++++++
    //   help: if this is a type, explicitly ignore the parameter name
    //      |
    //      | fn those_functions(_: x) {
    //      |
    println!("x is {}", x);
}

// function signature having multiple, typed parameters
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

// ## Functions that return values must have a type declared after `->`
fn add_one(x: i32) -> i32 {
    x + 1
}

// ## Example of a function that's supposed to return a value, but where
//   the function body ends with a statement
//   (due to the use of a semicolon `;`).
//   Remember that `()` is the unit type.
// fn less_one(x: i32) -> i32 {
//     // mismatched types
//     // expected `i32`, found `()`
//     x - 1;
// }
