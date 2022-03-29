use std::{env, process};

use minigrep::Config;

// cargo run -q frog ../../../poem.txt
// How public, like a frog
//
// cargo run -q body ../../../poem.txt
// I'm nobody! Who are you?
// Are you nobody, too?
// How dreary to be somebody!
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
