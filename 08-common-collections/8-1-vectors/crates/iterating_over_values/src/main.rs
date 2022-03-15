fn main() {
    // (A) iterate through each element with a for loop
    let v = vec![100, 32, 57];
    for i in &v {
        println!("i is {}", i);
    }
    // i is 100
    // i is 32
    // i is 57

    // (B)
    // iterate over mutable references to each element in a mutable vector
    let mut v = vec![100, 32, 57];
    println!("original v is {:?}", v);
    for i in &mut v {
        // i is of type &mut i32

        println!("original i is {}", i);

        // i += 50;
        // error[E0368]: binary assignment operation `+=` cannot be applied to type `&mut {integer}`
        // `+=` can be used on `{integer}`, you can dereference `i`
        // *i += 50;

        // To change the value that the mutable reference refers to, use the
        // dereference operator `*` to get the value in `i` before we can use
        // the `+=` operator.
        *i += 50;
        println!("mutated i is {}", i);
    }
    println!("v is now {:?}", v);
    // original v is [100, 32, 57]
    // original i is 100
    // mutated i is 150
    // original i is 32
    // mutated i is 82
    // original i is 57
    // mutated i is 107
    // v is now [150, 82, 107]
}
