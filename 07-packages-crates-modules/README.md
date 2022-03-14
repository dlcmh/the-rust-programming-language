# 7. Managing Growing Projects with Packages, Crates, and Modules

[code sample](./libs/restaurant/src/lib.rs)

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

The standard library (`std`) is shipped with the Rust language, hence isn't necessary to be included in Cargo.toml. But we do need to referto it with `use` to bring items from there into our package's scope, eg `use std::collections::HashMap;`.

## Use Nested Paths to clean up large `use` lists

```rust
// (A) instead of:
use std::cmp::Ordering;
use std::io;

// write:
use std::{cmp::Ordering, io};

// (B) instead of:
use std::io;
use std::io::Write;

// write:
use std::io{self, Write};
```

## The Glob Operator, \*

To bring in all public items defined in a path into scope:

```rust
use std::collections::*;
```

Be careful when using `*` as it makes it harder to tell what names are in scope and where a name was defined.

`*` is often used when testing to bring everything under test into the `tests` module.

See also the [prelude pattern in the standard library docs](https://doc.rust-lang.org/std/prelude/index.html#other-preludes).

## Separating modules into different files

[source code](./bins/restaurant_runner/src/)
