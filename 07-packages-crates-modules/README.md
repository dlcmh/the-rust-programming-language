# 7. Managing Growing Projects with Packages, Crates, and Modules

The module tree for the [restaurant library crate](./libs/restaurant/) is:

```
src/lib.rs (this file is the "crate root" for the implicit module named "crate")
 └── front_of_house (`mod`)
     ├── hosting (`mod`)
     │   ├── add_to_waitlist (`fn`)
     │   └── seat_at_table (`fn)
     └── serving (`mod`)
         ├── take_order (`fn`)
         ├── serve_order (`fn`)
         └── take_payment (`fn`)
```

Note that the entire module tree is rooted under the implicit module named "crate".

## Rust privacy

All items are private by default:

- functions
- methods
- structs
- enums
- modules
- constants

### Parent and child modules

(A) Items in a parent module aren't allowed to access private items in child modules.

(B) But items in child modules can use items in their ancestor modules.

Reasons for (A) & (B):

- allow child modules to wrap and hide their implementation details
- allow child modules to see the context in which they're defined
- allow programmer to know which parts of inner code can be changed without breaking outer code

## External packages

THe standard library (`std`) is shipped with the Rust language, hence isn't necessary to be included in Cargo.toml. But we do need to referto it with `use` to bring items from there into our package's scope, eg `use std::collections::HashMap;`.
