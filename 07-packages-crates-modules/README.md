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
