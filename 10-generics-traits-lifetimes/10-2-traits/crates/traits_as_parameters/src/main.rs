pub trait Summary {
    fn summarize(&self) -> String;
}
pub struct Item {
    pub name: String,
}
impl Summary for Item {
    fn summarize(&self) -> String {
        format!("<Item: {}>", self.name)
    }
}
pub struct Element {
    pub name: String,
}
impl Summary for Element {
    fn summarize(&self) -> String {
        format!("<Element: {}>", self.name)
    }
}

// (A) - `impl Trait` syntax
// - `item` parameter accepts any type that implements the `Summary` trait
// - syntax sugar for (B)
fn f1(item: &impl Summary) {
    println!("f1: {}", item.summarize());
}

// (B) - Trait bound syntax
fn f2<T: Summary>(item: &T) {
    println!("f2: {}", item.summarize());
}

// (C) - `item1` & `item2` must both implement `Summary`, but could have different types
fn f3(item1: &impl Summary, item2: &impl Summary) {
    println!("f3: {}, {}", item1.summarize(), item2.summarize())
}

// (D) - use trait bound to enforce both items to have the same type
fn f4<T: Summary>(item1: &T, item2: &T) {
    println!("f4: {}, {}", item1.summarize(), item2.summarize())
}

fn main() {
    let item = Item {
        name: String::from("item"),
    };
    let element = Element {
        name: String::from("element"),
    };

    f1(&item);
    // f1: <Item: item>

    f2(&element);
    // f2: <Element: element>

    f3(&item, &element);
    // f3: <Item: item>, <Element: element>

    // f4(&item, &element);
    //     error[E0308]: mismatched types
    //     --> src/main.rs:60:15
    //      |
    //   60 |     f4(&item, &element);
    //      |               ^^^^^^^^ expected struct `Item`, found struct `Element`
    //      |
    //      = note: expected reference `&Item`
    //                 found reference `&Element`

    f4(&item, &item);
    // f4: <Item: item>, <Item: item>
    f4(&element, &element);
    // f4: <Element: element>, <Element: element>
}
