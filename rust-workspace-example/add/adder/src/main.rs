use add_one;
use rand::prelude::*;

fn main() {
    // We can use random() immediately. It can produce values of many common types:
    let random_number: u8 = random();
    println!("{}", random_number);

    let num = 10;
    println!(
        "Hello, world! {} plus one is {}!",
        num,
        add_one::add_one(num)
    );
}
