# 10.1. Generic data types

Generics can be used to define:

- [functions](./crates/generics_in_functions/src/main.rs)
  - generics are placed in the signature of the function - where data types of parameters and the return value are specified
- [structs](./crates/generics_in_structs/src/main.rs)
- [enums](./crates/generics_in_enums/src/main.rs)
- [methods](./crates/generics_in_methods/src/main.rs)

## How generics affect code performance.

Rust uses monomorphization, which is the process of turning generic code into specific code by filling in the concrete types that are used when compiled, to ensure that there is no additional runtime cost associated with using generic type parameters.
