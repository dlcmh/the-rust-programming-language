use std::fs::File;

fn main() {
    // f is inferred as type Result<File, Error>
    // `File` is a file handle that we can read from or write to
    let f = File::open("no-such-file.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    // thread 'main' panicked at 'Problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:10:23
}
