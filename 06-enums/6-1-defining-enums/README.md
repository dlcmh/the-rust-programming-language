# 6.1. Defining an enum

## Code samples

[define enums](./crates/define_enums/src/main.rs)

### The `Option` enum

- encodes common scenario where a value could be something, or nothing (eg request the first item of an empty list)
- expressing this concept in terms of the type system means the compiler can check if all cases have been handled
- prevents bugs that are extremely common in other programming languages

Definition in the standard library:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

[Option enum vs null](./crates/option_vs_null/src/main.rs)
