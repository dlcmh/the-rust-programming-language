# 9.2. Recoverable errors with `Result`

The `Result` enum is defined as having 2 variants:

```rust
#![allow(unused)]
fn main() {
    // `T` and `E` are generic type parameters
    // - `T`: type of value returned in a success case within `Ok` variant
    // - `E`: type of error in a failure case within the `Err` variant
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
}
```

## Code samples

[open a file](./crates/open_file/src/main.rs)

[match on different errors with `ErrorKind`](./crates/match_on_error_kind/src/main.rs)

[unwrap vs expect - both call `panic!` macro on error](./crates/unwrap_expect/src/main.rs)

[propagate error back to caller](./crates/propagate_error/src/main.rs)

[`?` fails on `main`](./crates/question_mark_operator/fails_on_main/src/main.rs)

[`?` on `main` with `Box<dyn Error>` return type](./crates/question_mark_operator/ok_on_main/src/main.rs)
