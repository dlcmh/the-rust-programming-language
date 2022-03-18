use std::collections::HashMap;

fn main() {
    let sequences: HashMap<_, _> = vec![("one".to_string(), 1), ("two".to_string(), 2)]
        .into_iter()
        .collect();

    // (A) get a value out of the hash map by providing its key to the `get` method
    let wanted_key = String::from("two");
    let sequence_number = sequences.get(&wanted_key);
    println!(
        "number (of type `Option<&i32`) for {} is {:?}",
        wanted_key, sequence_number
    );
    // number (of type `Option<&i32`) for two is Some(2)
    // The result, 2, is wrapped in `Some` because `get` returns an `Option<&V>`,
    // if no value exists for that key, `get` will return `None`.
    let wanted_key = String::from("Wookiee");
    let sequence_number = sequences.get(&wanted_key);
    println!(
        "number (of type `Option<&i32`) for {} is {:?}",
        wanted_key, sequence_number
    );
    // number (of type `Option<&i32`) for Wookiee is None

    // (B) iterate over each key/value pair
    // &sequences - so that ownership of sequences is NOT moved into this for loop
    for (key, value) in &sequences {
        println!("{}: {}", key, value);
    }
}
