mod front_of_house;

use front_of_house::{front1, reception};

use back_of_house::{self, back1, utilities};

fn main() {
    println!("Hello, world!");

    front1();

    reception::reception1();

    back1();

    utilities::cleanup1();
}
