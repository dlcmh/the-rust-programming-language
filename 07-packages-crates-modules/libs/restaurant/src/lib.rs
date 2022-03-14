// eat_at_restaurant & front_of_house are siblings because both items are
// defined in the same module. Each can access or refer to the other.
mod front_of_house {
    // (A)
    // not accessible by eat_at_restaurant
    // mod hosting {

    // (B)
    // accessible by eat_at_restaurant when marked with `pub` keyword
    pub mod hosting {
        // (C)
        // not accessible by eat_at_restaurant
        // fn add_to_waitlist() {}

        // (D)
        // accessible by eat_at_restaurant when marked with `pub` keyword
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // ## Making structs public

    // Order a breakfast is the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    println!("I'd like {} toast please.", meal.toast); // I'd like Rye toast please.

    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please.", meal.toast); // I'd like Wheat toast please.

    // field `seasonal_fruit` of struct `Breakfast` is private
    // private field rustc(E0616)
    // meal.seasonal_fruit = String::from("papaya");

    // All variants of a public enum are public by default
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    println!("{:?}", order1); // Soup
    println!("{:#?}", order2); // Salad
}

// Bringing paths into scope with the `use` keyword
// Any of (E), (F), (G) are acceptable ways:
// (E)
use crate::front_of_house::hosting;
// (F)
// use self::front_of_house::hosting;
// (G)
// use front_of_house::hosting;

// (H)
// The idiomatic way of bringing a function into scope is to call
// use crate::front_of_house::hosting followed by calling the
// hosting::add_to_waitlist() the function, that is, to call the parent module
// followed by  calling the function - this makes it clear that the function
// isn't locally defined while still minimizing the repetition of the full path.
pub fn dine_at_restaurant() {
    // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    // Relative path
    // front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// (I)
// When bringing in structs and enums, it's idiomatic to specify the full path
use std::collections::HashMap;

fn use_hash_map() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("{:?}", map); // {1: 2}
}

// (J)
// Bringing two types with the same name into the same scope requires using
// their parent modules.
use std::fmt; // NOT std::fmt::Result
use std::io; // NOT std::io::Result

// fn function1() -> fmt::Result {}
// fn function2() -> io::Result<()> {}

// (K)
// alternatively, choose a new name for one of the types:
use std::io::Result as IoResult;
// fn function3() -> IoResult<()> {}

// referred to by fix_incorrect_order via `super`
// use `super` if we think serve_order & back_of_house are likely to stay
// in the same relationship with each other and get moved together should
// we decide to reorganize the crate's module tree
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        // Struct fields are private by default, unless marked with `pub`
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // because the Breakfast struct has a private field, the struct needs
        // to provide a public associated function that constructs an instance
        // of Breakfast:
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // println! an enum requires:
    #[derive(Debug)]
    //     error[E0277]: `Appetizer` doesn't implement `Debug`
    //     --> src/lib.rs:55:22
    //      |
    //   55 |     println!("{:?}", order1); // Soup
    //      |                      ^^^^^^ `Appetizer` cannot be formatted using `{:?}`
    //      |
    //      = help: the trait `Debug` is not implemented for `Appetizer`
    //      = note: add `#[derive(Debug)]` to `Appetizer` or manually `impl Debug for Appetizer`
    //      = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
    // unlike a struct, all of the variants of a public enum are public by
    // default
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn eat_at_restaurant() {
        crate::eat_at_restaurant();
    }

    #[test]
    fn use_hash_map() {
        crate::use_hash_map();
    }
}
