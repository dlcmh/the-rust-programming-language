use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    // cargo run searchstring example-filename.txt
    // println!("args: {:?}", args);
    // args: ["/Users/dlcmh/rust/target/debug/minigrep", "searchstring", "example-filename.txt"]

    let query = &args[1];
    let filename = &args[2];
    println!("Searching for {}", query);
    println!("In file {}", filename);
    // cargo run searchstring example-filename.txt
    // Searching for searchstring
    // In file example-filename.txt

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);
}
