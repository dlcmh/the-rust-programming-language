// (A) & (B) differ only in their names and types in their signatures:
// - their function bodies have the same code
// -> eliminate duplication by creating a single function, (C), with a generic type parameter

// (A)
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// (B)
fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// (C)
// `T` in `largest<T>` is the name of the type parameter, which needs to be
// declared this way, inside angle brackets `<>`, before being used.
// Read as: the function `largest` is generic over some type `T`. This function
// has one parameter named `list`, which is a slice of values of type `T`. The
// function will return a value of the same type `T`.
//
// Error #1:
// - `std::cmp::PartialOrd` is a trait
// error[E0369]: binary operation `>` cannot be applied to type `T`
//   --> src/main.rs:57:17
//    |
// 57 |         if item > largest {
//    |            ---- ^ ------- &T
//    |            |
//    |            T
//    |
// help: consider restricting type parameter `T`
//    |
// 52 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> T {
//    |             ++++++++++++++++++++++
fn largest<T>(list: &[T]) -> T {
    let mut largest = &list[0];

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
