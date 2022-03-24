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

fn f1(item: &impl Summary) {
    println!("f1: {}", item.summarize());
}

fn main() {
    let i1 = Item {
        name: String::from("i1"),
    };
    f1(&i1);
    // f1: i1
}
