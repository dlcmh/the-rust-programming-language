use std::fmt::{self, Display};

// https://doc.rust-lang.org/std/fmt/trait.Display.html
struct User {
    name: String,
    email: String,
}
impl Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "User with name: {}, email: {})", self.name, self.email)
    }
}

// syntax specifying all the following in one function:
// - generic type parameters
// - trait bounds
// - lifetimes
//
// 1. The function returns the longer of two string slices.
//
// 2. Parameter `ann` has generic type `T` accepts any type that implements the `Display`
// trait as specified by the `where` clause. This extra parameter will be printed using
// `{}`, which is why the `Display` trait bound is necessary.
//
// 3.Because lifetimes are a type of generic, the declarations, of the lifetime parameter
// `'a` and the generic type parameter `T` go in the same list inside the angle brackets
// after the function name.
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let user = User {
        name: "David".to_string(),
        email: "email@test.com".to_string(),
    };
    println!("{}", user);
    // User with name: David, email: email@test.com)

    let longest = longest_with_an_announcement(&user.name, &user.email, &user);
    println!("{}", longest);
    // Announcement: User with name: David, email: email@test.com)
    // email@test.com
}
