pub trait Summary {
    fn summarize(&self) -> String;
}
pub struct Item {
    pub name: String,
}
impl Summary for Item {
    fn summarize(&self) -> String {
        format!("{}", self.name)
    }
}

// (A) - `item` parameter accepts any type that implements the `Summary` trait
// - syntax sugar for (B)
fn f1(item: &impl Summary) {
    println!("f1: {}", item.summarize());
}

// (B) - Trait bound syntax
fn f2<T: Summary>(item: &T) {
    println!("f2: {}", item.summarize());
}

fn main() {
    let i1 = Item {
        name: String::from("i1"),
    };
    f1(&i1);
    // f1: i1
    f2(&i1);
}
