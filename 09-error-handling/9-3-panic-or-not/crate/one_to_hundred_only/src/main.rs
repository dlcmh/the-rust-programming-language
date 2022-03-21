mod inner {
    // make a new, custom `Guess` type that will only create an instance of `Guess`
    // if the `new` function receives a value between ` and `00

    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value myst be between 1 and 100, got {}.", value);
            }

            Guess { value }
        }

        // define a public getter, which is required because the `value` field is private
        pub fn value(&self) -> i32 {
            self.value
        }
    }
}

use inner::Guess;

fn main() {
    // println!("{}", Guess::new(-1).value());
    // thread 'main' panicked at 'Guess value myst be between 1 and 100, got -1.', src/main.rs:11:13
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    // println!("{}", Guess::new(101).value());
    // thread 'main' panicked at 'Guess value myst be between 1 and 100, got 101.', src/main.rs:11:13
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    println!("{}", Guess::new(77).value()); // 77
}
