fn main() {
    let s1 = String::from("Hello");
    let h = s1[0];
    // error[E0277]: the type `String` cannot be indexed by `{integer}`
    //  --> src/main.rs:3:13
    //   |
    // 3 |     let h = s1[0];
    //   |             ^^^^^ `String` cannot be indexed by `{integer}`
    //   |
    //   = help: the trait `Index<{integer}>` is not implemented for `String`
}
