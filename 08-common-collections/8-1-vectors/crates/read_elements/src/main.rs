fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // (A)
    let third = &v[2];
    println!("The third element is {}", third); // The third element is 3

    // (B)
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    // The third element is 3

    // (C)
    match v.get(20) {
        Some(nineteenth) => println!("The nineteenth element is {}", nineteenth),
        None => println!("There is no nineteenth element."),
    }
    // There is no nineteenth element.
}
