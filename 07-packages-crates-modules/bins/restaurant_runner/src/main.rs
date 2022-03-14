mod back_of_house;
mod front_of_house;

use front_of_house::{front1, reception};

use back_of_house::{
    back1,
    disinfection::{
        disinfect1,
        supplies::{purchase as purchase_supplies, replenish as replenish_supplies},
    },
    utilities,
};

fn main() {
    println!("Hello, world!");

    front1();

    reception::reception1();

    back1();

    utilities::cleanup1();

    disinfect1();

    purchase_supplies();

    replenish_supplies();

    // Hello, world!
    // front1!
    // reception1!
    // back1!
    // cleanup1!
    // disinfect!
    // purchase disinfection supplies!
    // replenish disinfection supplies!
}
