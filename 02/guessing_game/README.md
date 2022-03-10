## gen_range

`1..101` & `1..=100` are equivalent, which means the range of 1 to 100, inclusive of 1 and 100.

## Documentations

`cargo doc --open` generates documentation for all dependencies specified in Cargo.toml, and also your own code.

## match

`Err(_) => ...` - the underscore `_` is a catchall value; we're saying we want to match all `Err` values, no matter what information they have inside them
