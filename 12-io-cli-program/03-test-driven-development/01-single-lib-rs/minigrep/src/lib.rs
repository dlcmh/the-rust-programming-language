use std::{error::Error, fs};

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

// cargo run -q frog ../../poem.txt
// How public, like a frog
//
// cargo run -q body ../../poem.txt
// I'm nobody! Who are you?
// Are you nobody, too?
// How dreary to be somebody!
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line)
    }

    Ok(())
}

// The explicit lifetime `'a` specify which argument lifetime is connected to the lifetime
// of the return value.
// `'a` indicate that the returned vector should contain string slices that reference
// slices of the argument `contents` instead of the argument `query`.
// The data returned by `search` will live as long as the data passed in the `contents` argument.
// Otherwise, error:
// ...this function's return type contains a borrowed value,
// but the signature does not say whether it is borrowed from `query` or `contents`
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        // `\` backslash tells Rust not to put a newline character at the beginning
        // of the contents of this string literal
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
