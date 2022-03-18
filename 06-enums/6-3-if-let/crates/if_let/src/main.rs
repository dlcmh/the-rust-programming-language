use std::mem::size_of_val;

fn main() {
    // https://doc.rust-lang.org/rust-by-example/types/literals.html
    // numeric literals can be type-annotated by adding the tupe as a suffix;
    // in the example below, the literal is `3` while its type is `u8`
    let m = 3u8;
    let n = 333_i32;
    println!(
        "number {} of type `u8` has size {} byte(s)",
        m,
        size_of_val(&m)
    );
    println!(
        "number {} of type `i32` has size {} byte(s)",
        n,
        size_of_val(&n)
    );
    // number 3 of type `u8` has size 1 byte(s)
    // number 333 of type `i32` has size 4 byte(s)

    // # `match` vs `if let`
    enum Vote {
        Yes(u8),
        No,
    }
    let mut count = 0;

    // Alternative 1: match
    let yes = Vote::Yes(3);
    match yes {
        Vote::Yes(how_many) => {
            count += how_many;
        }
        _ => {}
    }
    println!("count after `match` is {}", count);
    // count after `match` is 3

    // Alternative 2: if let
    let yes = Vote::Yes(10);
    if let Vote::Yes(how_many) = yes {
        count += how_many;
    }
    println!("count after `if let` is {}", count);
    // count after `if let` is 13
}
