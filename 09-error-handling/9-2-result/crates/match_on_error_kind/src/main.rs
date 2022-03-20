use std::{fs::File, io::ErrorKind};

fn main() {
    // (A) verbose pattern using `match`es
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,

        // `error` is a struct of type `io::Error` provided by the standard library;
        // `io::Error` has a method `kind` that can be called to give an
        // `ErrorKind` enum.s
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("Creating file...");
                match File::create("hello.txt") {
                    Ok(fc) => {
                        println!("File created!");
                        fc
                    }
                    Err(e) => panic!("Problem creating file: {:?}", e),
                }
            }
            other_error => panic!("Problem opening file: {:?}", other_error),
        },
    };
    println!("(A) File handle: {:#?}", f);
    // (A) File handle: File {
    //     fd: 3,
    //     path: "/Users/dlcmh/projects/the-rust-programming-language/09-error-handling/9-2-result/crates/match_on_error_kind/hello.txt",
    //     read: true,
    //     write: false,
    // }

    // (B) more concise using methods on `Result<T, E> that accept closures
    let f = File::open("bye.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("bye.txt").unwrap_or_else(|error| {
                panic!("Problem creating file: {:?}", error);
            })
        } else {
            panic!("Problem opening file: {:?}", error);
        }
    });
    println!("(B) File handle: {:#?}", f);
    // (B) File handle: File {
    //     fd: 4,
    //     path: "/Users/dlcmh/projects/the-rust-programming-language/09-error-handling/9-2-result/crates/match_on_error_kind/bye.txt",
    //     read: false,
    //     write: true,
    // }
}
