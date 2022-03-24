pub trait Summary {
    fn summarize(&self) -> String;
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// `returns_summarizable` returns a value of some type that implements the
// `Summary` trait without naming the concrete type.
// In this example, the code calling `returns_summarizable` won't know that
// a `Tweet` is being returned.
//
// This functionality is useful within the context of closures and iterators,
// which will be covered in Chapter 13. The `impl Trait` syntax lets us concisely
// specify that a function returns some type that implements the `Iterator` trait
// without needing to write out a very long type.
//
// Note however that this syntax can only be used to return a single type.
pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
fn main() {
    let summarizable = returns_summarizable();

    println!("{}", summarizable.summarize());
    // horse_ebooks: of course, as you probably already know, people
}
