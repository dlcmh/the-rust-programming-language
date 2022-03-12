fn main() {
    // ## Count up using `Range` (provided by the standard library)
    for count in 1..4 {
        println!("{}...", count);
    }
    println!("Done!");
    // 1...
    // 2...
    // 3...
    // Done!

    // ## `if` and `else` expression arms must return the same value types
    println!("Gender: {}", if 1 > 10 { "Male" } else { "Female" });
    // Gender: Female

    // println!("Guessed: {}", if 1 > 10 { "Male" } else { 6 });
    // error[E0308]: `if` and `else` have incompatible types
    // --> src/main.rs:16:57
    //  |
    //  |     println!("Guessed: {}", if 1 > 10 { "Male" } else { 6 });
    //  |                                         ------          ^ expected `&str`, found integer
    //  |                                         |
    //  |                                         expected because of this

    // ## step_by(), and inclusive of Range's second number
    let max_steps = 10;
    print!("Took ");
    for count in (2..=max_steps).step_by(2) {
        print!("{} {}", count, if count < max_steps { "~~ " } else { "" });
    }
    println!("steps!");
    // Took 2 ~~ 4 ~~ 6 ~~ 8 ~~ 10 steps!

    // ## Count down by calling `rev` on the range
    // `rev()` reverses an iterator's direction.
    for count in (1..4).rev() {
        print!("{} >> ", count);
    }
    println!("LIFTOFF!");
    // 3 >> 2 >> 1 >> LIFTOFF!
}
