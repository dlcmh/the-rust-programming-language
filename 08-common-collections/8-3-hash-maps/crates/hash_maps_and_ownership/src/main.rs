use std::collections::HashMap;

fn main() {
    let field_name = String::from("Favorite number");
    let field_value = 1;

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("After insertion, map is {:?}", map); // After insertion, map is {"Favorite number": 1}

    // ## For types like `i32` that implement the `Copy` trait, they get copied into the hash map
    println!("After insertion, field_value is {}", field_value); // After insertion, field_value is 1

    // ## For owned values like `String`, the values will be moved and the hash map becomes the new owner
    // println!("After insertion, field_name is {}", field_name);
    // error[E0382]: borrow of moved value: `field_name`
    //   --> src/main.rs:15:51
    //    |
    // 4  |     let field_name = String::from("Favorite number");
    //    |         ---------- move occurs because `field_name` has type `String`, which does not implement the `Copy` trait
    // ...
    // 8  |     map.insert(field_name, field_value);
    //    |                ---------- value moved here
    // ...
    // 15 |     println!("After insertion, field_name is {}", field_name);
    //    |                                                   ^^^^^^^^^^ value borrowed here after move
    //    |
}
