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
}

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
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
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
}
