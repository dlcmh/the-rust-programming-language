# Common Programming Concepts

## Variables

By default, variables are immutable.

```rust
let w = 5; // create immutable variable y and bind y to a value of 5
let mut x = 5; // create mutable variable x and bind x to a value of 5
```

## Constants

Unlike variables, constants:

- don't allow the use of `mut`
- can be declated in any scope, including the global scope
- must be annotated with the type of value

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```
