use std::fmt::{self, Display};

pub trait Summary {
    fn summarize(&self) -> String;
}
pub struct Item {
    pub name: String,
}
impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = &self.name;
        write!(f, "twice now: ({name}, {name})")
    }
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
pub fn f1(item: &impl Summary) {
    println!("f1: {}", item.summarize());
}

// (B) - Trait bound syntax
pub fn f2<T: Summary>(item: &T) {
    println!("f2: {}", item.summarize());
}

// (C) - `item1` & `item2` must both implement `Summary`, but could have different types
pub fn f3(item1: &impl Summary, item2: &impl Summary) {
    println!("f3: {}, {}", item1.summarize(), item2.summarize())
}

// (D) - use trait bound to enforce both items to have the same type
pub fn f4<T: Summary>(item1: &T, item2: &T) {
    println!("f4: {}, {}", item1.summarize(), item2.summarize())
}

// (E-1) - multiple trait bounds with `+`
// - `item` must implement both `Display` & `Summary`
pub fn f5(item: &(impl Display + Summary)) {
    println!("f5: {} / <{}>", item.summarize(), item)
}

// (E-2) - multiple trait bounds with `+`
// - also valid with trait bounds on generic types
pub fn f6<T: Summary + Display>(item: &T) {
    println!("f6: {} / <{}>", item.summarize(), item)
}

// (F) - clearer trait bounds with `where` clauses
pub fn f7<T, U>(item1: &T, item2: &U)
where
    T: Display + Summary,
    U: Summary,
{
    println!(
        "f7: {} / <{}> || {}",
        item1.summarize(),
        item1,
        item2.summarize()
    )
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
    //     --> src/main.rs:94:15
    //      |
    //   94 |     f4(&item, &element);
    //      |               ^^^^^^^^ expected struct `Item`, found struct `Element`
    //      |
    //      = note: expected reference `&Item`
    //                 found reference `&Element`

    f4(&item, &item);
    // f4: <Item: item>, <Item: item>
    f4(&element, &element);
    // f4: <Element: element>, <Element: element>

    // f5(&element);
    //     error[E0277]: `Element` doesn't implement `std::fmt::Display`
    //     --> src/main.rs:109:8
    //       |
    //   109 |     f5(&element);
    //       |     -- ^^^^^^^^ `Element` cannot be formatted with the default formatter
    //       |     |
    //       |     required by a bound introduced by this call
    //       |
    //       = help: the trait `std::fmt::Display` is not implemented for `Element`
    //       = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
    //    note: required by a bound in `f5`
    //      --> src/main.rs:53:24
    //       |
    //    53 | pub fn f5(item: &(impl Display + Summary)) {
    //       |                        ^^^^^^^ required by this bound in `f5`

    f5(&item);
    // f5: <Item: item> / <twice now: (item, item)>

    // f6(&element);
    //     error[E0277]: `Element` doesn't implement `std::fmt::Display`
    //     --> src/main.rs:129:8
    //      |
    //  129 |     f6(&element);
    //      |     -- ^^^^^^^^ `Element` cannot be formatted with the default formatter
    //      |     |
    //      |     required by a bound introduced by this call
    //      |
    //      = help: the trait `std::fmt::Display` is not implemented for `Element`
    //      = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
    //  note: required by a bound in `f6`
    //     --> src/main.rs:59:24
    //      |
    //  59  | pub fn f6<T: Summary + Display>(item: &T) {
    //      |                        ^^^^^^^ required by this bound in `f6`

    f6(&item);
    // f6: <Item: item> / <twice now: (item, item)>

    f7(&item, &element);
    // f7: <Item: item> / <twice now: (item, item)> || <Element: element>
}
