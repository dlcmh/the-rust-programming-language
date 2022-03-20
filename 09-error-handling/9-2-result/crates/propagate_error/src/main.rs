use std::{
    fs::File,
    io::{Error, Read},
};

// (A) return early on error
// `io::Error` is chosen as the return type of this function because that's the
// type of error value returned from ALL operations being called in the body,
// `File::open` & `read_to_string` method.
fn read_file_a() -> Result<String, Error> {
    // op #1
    let f = File::open("a.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e), // return early with error
    };

    // op #2
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// (B) `?` operator eliminates boilerplate and makes returning from functions on error easier
// error values that have `?` called on them gets converted with the `from` function
// into the return type of the function; this can happen as long as there's an
// `impl From<OtherError> for ReturnedError` to define the conversion in the trait's
// `from` function.
fn read_file_b() -> Result<String, Error> {
    let mut f = File::open("b.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// (C) shorten (B) by chaining calls immediately after `?`
fn read_file_c() -> Result<String, Error> {
    let mut s = String::new();
    File::open("c.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    println!("{}", read_file_a().unwrap()); // Hello, A!
    println!("{}", read_file_b().unwrap()); // Hello, B!
    println!("{}", read_file_c().unwrap()); // Hello, C!
}
