use std::{error::Error, fs};

mod config;
mod search;

pub use config::Config; // so we can `use minigrep::Config` in `main.rs`
use search::search; // otherwise we'd have to call `search::search` everywhere

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line)
    }

    Ok(())
}
