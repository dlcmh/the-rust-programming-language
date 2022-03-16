# 8.2. Storing UTF-8 encoded text with strings

Read [Why your first FizzBuzz implementation may not work - an article by Chris Morgan](https://chrismorgan.info/blog/rust-fizzbuzz/) on why Rust has 2 types of strings, unlike C#, Java, & Go (Golang) which have only 1.

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

[A `String` cannot be indexed by `{integer}`](./crates/cannot_index_string/src/main.rs)

[Internal representation - a `String` is a wrapper over a `Vec<u8>`](./crates/string_wraps_vec_u8/src/main.rs)

[Properly iterate over the characters / grapheme clusters](./crates/iterating_over_strings/src/main.rs)
