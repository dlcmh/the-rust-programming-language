#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Option<UsState>),
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        // https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#-bindings
        // https://doc.rust-lang.org/rust-by-example/flow_control/match/binding.html
        // `match` provides the `@` sigil for binding matched values to a variable
        m @ Coin::Penny => {
            println!("Lucky penny! (matched: {:?})", m);
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,

        // this match arm binds to the parts of the values that match the pattern;
        // the variable `binds` bunds to the value of the quarter's state
        Coin::Quarter(optional_state) => {
            // matching with `Option<T>`
            match optional_state {
                None => {}
                // matches are exhaustive - if we had omitted `None => {}`...
                // error[E0004]: non-exhaustive patterns: `None` not covered
                //    --> src/main.rs:27:19
                //     |
                // 27  |             match optional_state {
                //     |                   ^^^^^^^^^^^^^^ pattern `None` not covered
                //     |
                //    ::: /Users/dlcmh/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/option.rs:519:5
                //     |
                // 519 |     None,
                //     |     ---- not covered
                //     |
                //     = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
                //     = note: the matched value is of type `Option<UsState>`
                Some(state) => {
                    println!("State quarter from {:?}!", state);
                }
            }
            25
        }
    }
}

fn main() {
    println!("{}", value_in_cents(Coin::Penny));
    println!("{}", value_in_cents(Coin::Nickel));
    println!("{}", value_in_cents(Coin::Dime));
    println!(
        "Quarter with US state: {}",
        value_in_cents(Coin::Quarter(Some(UsState::Alabama)))
    );
    println!(
        "Quarter without US state: {}",
        value_in_cents(Coin::Quarter(None))
    );
    // Lucky penny! (matched: Penny)
    // 1
    // 5
    // 10
    // State quarter from Alabama!
    // Quarter with US state: 25
    // Quarter without US state: 25
}
