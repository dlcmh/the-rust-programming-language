use std::{env, error::Error, fs, process};

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

// let `main` handle errors by returning `Result<T, E>`
//
// `Box<dyn Error>` is a trait object
// - a type that implements the `Error` trait
// - there's no need to specify what particular type the return value will be
// - `dyn` is short for dynamic
//
// The `()` in `Ok(())` is an idiomatic way of indicating that `run` is being called
// for its side effects only; there's no return value that is needed from it.
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // `?` lets the caller handle any error value returned
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // handle the error with `if let` instead of `unwrap_or_else` to only exit
    // with `1` if there's an error.
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
        // cargo run -q searchstring no-such-file.txt
        // Searching for searchstring
        // In file no-such-file.txt
        // Application error: No such file or directory (os error 2)
    }
}
