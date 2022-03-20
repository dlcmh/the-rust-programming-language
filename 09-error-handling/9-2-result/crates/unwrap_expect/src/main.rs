use std::fs::File;

fn main() {
    // unwrap - prints standard error message
    // let f = File::open("hello.txt").unwrap();
    // thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:4:37

    // expect - prints custom error message
    // let f = File::open("bye.txt").expect("Failed to open bye.txt");
    // thread 'main' panicked at 'Failed to open bye.txt: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:8:35
}
