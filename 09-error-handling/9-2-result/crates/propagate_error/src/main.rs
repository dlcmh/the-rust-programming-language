use std::{
    fs::File,
    io::{Error, Read},
};

// (A) return early on error
// `io::Error` is chosen as the return type of this function because that's the
// type of error value returned from ALL operations being called in the body,
// `File::open` & `read_to_string` method.
fn read_file_a() -> Result<String, Error> {
    let f = File::open("a.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e), // return early with error
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn main() {
    println!("{}", read_file_a().unwrap());
    // Hello, A!
}
