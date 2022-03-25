# 10.3. Validating references with lifetimes

TODO

## The static lifetime

`'static` means a reference can live for the entire duration of the program.

All string literals have the `'static` lifetime ([see also](../../08-common-collections/8-2-strings/README.md#the-string-type-in-the-core-language)), which can be annotated as follows:

```rust
let s: &'static str = "I have a static lifetime.";
```
