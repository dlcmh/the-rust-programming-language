use std::fs::File;

// `?` can only be used in functions that have a return type compatible with the value `?` is used on
// `File::open` returns `Result`, but `main` function returns `()`, resulting in a compilation error
fn main() {
    let _f = File::open("no-such-file.txt")?;
    //     error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
    //     --> src/main.rs:4:44
    //      |
    //    3 | / fn main() {
    //    4 | |     let _f = File::open("no-such-file.txt")?;
    //      | |                                            ^ cannot use the `?` operator in a function that returns `()`
    //    5 | | }
    //      | |_- this function should return `Result` or `Option` to accept `?`
    //      |
    //      = help: the trait `FromResidual<Result<Infallible, std::io::Error>>` is not implemented for `()`
}
