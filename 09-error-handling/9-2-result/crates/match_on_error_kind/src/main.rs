use std::{fs::File, io::ErrorKind};

fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
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
    println!("File handle: {:#?}", f);
    // File handle: File {
    //     fd: 3,
    //     path: "/Users/dlcmh/projects/the-rust-programming-language/09-error-handling/9-2-result/crates/match_on_error_kind/hello.txt",
    //     read: true,
    //     write: false,
    // }
}
