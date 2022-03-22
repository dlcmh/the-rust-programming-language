fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    println!(
        "largest of `vec![3, 2, 1]`: {}",
        largest_i32(&vec![3, 2, 1])
    );
    // largest of `vec![3, 2, 1]`: 3

    println!(
        "largest of `vec!['~', '!', 'a', 'Z']`: {}",
        largest_char(&vec!['~', '!', 'a', 'Z'])
    );
    // largest of `vec!['~', '!', 'a', 'Z']`: ~
}
