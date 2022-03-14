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
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
