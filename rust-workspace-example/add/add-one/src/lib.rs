use rand::prelude::*;

pub fn add_one(x: i32) -> i32 {
    // We can use random() immediately. It can produce values of many common types:
    let random_number: u8 = random();
    println!("{}", random_number);

    x + 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    use super::*;

    #[test]
    fn add_one_2_works() {
        assert_eq!(3, add_one(2));
    }

    #[test]
    fn add_one_3_works() {
        assert_eq!(4, add_one(3));
    }
}
