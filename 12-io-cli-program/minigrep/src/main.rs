use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);
    // cargo run searchstring example-filename.txt
    // args: ["/Users/dlcmh/rust/target/debug/minigrep", "searchstring", "example-filename.txt"]
}
