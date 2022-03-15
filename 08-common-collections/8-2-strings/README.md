# 8.2. Storing UTF-8 encoded text with strings

Strings are:

- implemented as a collection of bytes,
- plus some methods to provide useful functionality when those bytes are interpreted as text

### The string type in the core language

Rust has only one string type in the core language, which is the string slice `str` that is usually seen in its borrowed form `&str`.

String slices are references to some UTF-8 encoded string data stored elsewhere.

String literals, for example, are stored in the program's binary and are therefore string slices.

### The `String` type in the standard library

The `String` type, which is provided by Rust's standard library is a growable, mutable, owned, UTF-8 encoded string type.

TODO - ither string types in Rust's standard library:

- `OsString`
- `OsStr`
- `CString`
- `CStr`

Note that these names end in `String` or `Str`, which refer to owned and borrowed variants, just like the `String` and `str` types. They can store text in different encodings or be represented in memory in a different way.

## Code samples

[create string](./crates/create_string/src/main.rs)

[update string](./crates/update_string/src/main.rs)
