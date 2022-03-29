use std::{env, process};

use minigrep::Config;

// cargo run -q frog ../../../poem.txt
// How public, like a frog
//
// cargo run -q body ../../../poem.txt
// I'm nobody! Who are you?
// Are you nobody, too?
// How dreary to be somebody!
//
// CASE_INSENSITIVE=1 cargo run -q bODy ../../../poem.txt
// I'm nobody! Who are you?
// Are you nobody, too?
// How dreary to be somebody!
//
// (A) cargo run > output.txt - no content in output.txt, error displayed on the screen
// (B) cargo run -q frog ../../../poem.txt > output.txt - content in output.txt, nothig displayed on the screen
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
