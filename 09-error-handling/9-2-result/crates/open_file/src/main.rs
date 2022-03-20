use std::fs::File;

fn main() {
    // f is inferred as type Result<File, Error>
    let f = File::open("no-such-file.txt");
}
