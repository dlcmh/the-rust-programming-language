use std::{error::Error, fs::File};

// `Box<dyn Error>` type is called a trait object - in this context, it means "any kind of error"
// Rust executables follow the convention in C programs, which is to return `0` when exiting successfully,
// (`Ok(())` in the example below), and a nonzero integer otherwise.
//
// The types that `main` may return are those that implement the `std::process::Termination` trait
// which, at the time of writing, is only available in Nightly Rust.
fn main() -> Result<(), Box<dyn Error>> {
    let _f = File::open("might-exist.txt")?;
    Ok(())
}
// `cargo run -q`
// Error: Os { code: 2, kind: NotFound, message: "No such file or directory" }
