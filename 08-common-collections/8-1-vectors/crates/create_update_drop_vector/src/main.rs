fn main() {
    // (A)
    // let v_a = Vec::new();
    // type annotations needed for `Vec<T>`
    // cannot infer type for type parameter `T`rustc(E0282)
    // Type annotation is mandatory as vectors are implemented using generics.
    let v_a: Vec<i32> = Vec::new();
    println!("{:?}", v_a); // []

    // (B) - `vec!` macro
    let v_b = vec![1, 2, 3];
    println!("{:?}", v_b); // [1, 2, 3]

    // (C) only mutable vectors can be pushed to
    let mut v_c = Vec::new(); // no type annotations...
    v_c.push(1); // this first push sets the expected type of all elements

    // mismatched types
    // expected integer, found `&str`rustc(E0308)
    // v_b.push("1");
    println!("{:?}", v_c); // [1]

    // (D)
    // like all other structs, a vector is freed when it goes out of scope, and
    // all its content are also dropped (the values it holds are cleaned up)
    {
        let v_d = vec![1, 2];
        println!("{:?}", v_d); // [1, 2]
    }
    // println!("{:?}", v_d);
    // error[E0425]: cannot find value `v_d` in this scope
}
