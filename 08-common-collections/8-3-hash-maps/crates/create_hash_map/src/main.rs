use std::collections::HashMap;

fn main() {
    // (A)
    // `new` - create an empty hash map
    // `insert` - add elements
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("(A): {:?}", scores); // (A): {"Blue": 10, "Yellow": 50}

    // (B)
    // 2 vectors to be paired into an iterator of tuples ("zipped up" into a single iterator of pairs),
    // then converted into a hash map with the `collect` method. Type annotation `HashMap<_, _>` is
    // needed as Rust needs to know how to collect into the desired data structure. Using underscores
    // as the parameters for the key & value types allow Rust to infer the types that the hash map
    // containes based on the types of the data in the vectors.
    let teams = vec![String::from("Red"), String::from("Pink")];
    let initial_scores = vec![99, 33];
    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("(B): {:?}", scores); // (B): {"Pink": 33, "Red": 99}

    // (C)
    // https://stackoverflow.com/questions/30441698/how-do-i-create-a-map-from-a-list-in-a-functional-way
    // with Trait std::iter::Iterator > collect method
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect
    let tuples = [("one", 1), ("two", 2)];
    let sequences: HashMap<_, _> = tuples.into_iter().collect();
    println!("(C): {:?}", sequences); // (C): {"two": 2, "one": 1}
}
