fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // (A)
    let third = &v[2];
    println!("The third element is {}", third); // The third element is 3

    // use this method if you want the program to crash when referencing a non-existent element:
    // println!("The nineteenth element is {}", &v[20]);
    // thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 20'

    // (B)
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    // The third element is 3

    // (C)
    // `get` method with an index argument returns an `Option<T>` => `Some(&element)` or `None`
    match v.get(20) {
        Some(nineteenth) => println!("The nineteenth element is {}", nineteenth),
        None => println!("There is no nineteenth element."),
    }
    // There is no nineteenth element.

    // (D)
    // Borrow checker enforces ownership and borrowing rules. It's not allowed
    // to have mutable and immutable references in the same scope:
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];

    // v.push(6);
    // error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
    //   --> src/main.rs:33:5
    //    |
    // 31 |     let first = &v[0];
    //    |                  - immutable borrow occurs here
    // 32 |
    // 33 |     v.push(6);
    //    |     ^^^^^^^^^ mutable borrow occurs here
    // ...
    // 44 |     println!("The first element is {}", first);
    //    |                                         ----- immutable borrow later used here

    // The error is caused by the way vectors work: adding a new element onto
    // the end of the vector might require allocating new memory and copying the
    // old elements to the new space, if there isn't enough room to put all the
    // elements next to each other where the vector currently is. In that case,
    // the reference to the first element would be pointing to deallocated
    // memory. The borrowing rules prevent programs from ending up in that
    // situation.

    println!("The first element is {}", first);
}
