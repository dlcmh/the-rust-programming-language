fn main() {
    this_function();

    that_function(); // x is 2

    print_labeled_measurement(10, 'h') // The measurement is: 10h
}

fn this_function() {}

// function definition order isn't important
// notice `those_functions` are defined after `that_function`
fn that_function() {
    those_functions(2); // calling a function with a concrete value; the concrete value is called an "argument"
}

// x is a "parameter"
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
fn those_functions(x: i32) {
    println!("x is {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}
