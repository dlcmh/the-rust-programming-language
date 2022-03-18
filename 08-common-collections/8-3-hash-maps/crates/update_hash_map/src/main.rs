use std::collections::HashMap;

fn main() {
    // ## (A) overwrite a value
    let mut scores: HashMap<_, _> = vec![("Team one", 1)].into_iter().collect();
    println!("scores: {:?}", scores); // scores: {"Team one": 1}
    scores.insert("Team one", 20);
    println!("scores: {:?}", scores); // scores: {"Team one": 20}

    // ## (B) insert value only if the key has no value
    // `entry` returns an enum called `Entry` that represents a value that might exist.
    // The `or_insert` method on `Entry` returns:
    // - a mutable reference to the value for the corresponding `Entry` key if that key exists
    // - if the key doesn't exist, inserts the argument as the new value for this key
    //   and returns a mutable reference to the new value
    scores.entry("Team two").or_insert(33);
    println!("scores: {:?}", scores); // scores: {"Team one": 20, "Team two": 33}

    // Team two already has value 33, so Rust will NOT replace it with 55
    scores.entry("Team two").or_insert(55);
    println!("scores: {:?}", scores); // scores: {"Team one": 20, "Team two": 33}

    // ## (C) Updating a value based on the old one
    // `split_whitespace` iterates over sub-slices, separated by whitespace, of `text`
    let text = "hello world wonderful    world";
    let mut word_counts = HashMap::new();
    for word in text.split_whitespace() {
        // `or_insert` returns a mutable reference (`&mut V`) to the value for the specified key.
        // The mutable reference is stored in `count`, so in order to assign to that value,
        // we first dereference `count` with the asterisk (`*`).
        // The mutable reference goes out of scope at the end of the `for` loop, so all of these
        // changes are safe and allowed by the borrowing rules.
        let count = word_counts.entry(word).or_insert(0);
        *count += 1;
    }
    println!("word counts: {:?}", word_counts);
    // word counts: {"world": 2, "wonderful": 1, "hello": 1}
}
