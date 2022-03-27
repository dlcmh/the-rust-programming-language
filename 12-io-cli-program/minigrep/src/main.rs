use std::env;

// cargo run searchstring example-filename.txt
fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("args: {:?}", args);
    // args: ["/Users/dlcmh/rust/target/debug/minigrep", "searchstring", "example-filename.txt"]

    let query = &args[1];
    let filename = &args[2];
    println!("Searching for {}", query);
    println!("In file {}", filename);
    // Searching for searchstring
    // In file example-filename.txt
}
