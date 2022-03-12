fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    let immutable_shadow = "    ";
    println!("{}", immutable_shadow);

    let shadow = immutable_shadow.len();
    println!("{}", shadow);

    let mut mutable_shadow = 10;
    println!("{}", mutable_shadow);

    mutable_shadow = 20;
    println!("{}", mutable_shadow);

    let mutable_shadow = "40"; // not allowed to change type without use of `let` keyword
    println!("{}", mutable_shadow);
}
