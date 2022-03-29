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
