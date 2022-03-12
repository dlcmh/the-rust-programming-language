# 3.2 Data Types

## Numeric operations

[code sample](./numeric_operations/src/main.rs)

## The Boolean type

```rust
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```

## The Character type

Use single quotes:

```rust
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}
```

The `char` type is 4 bytes in size and represents a Unicode Scalar Value in the range of `U+0000` to `U+D7FF`, and `U+E000` to `U+10FFFF` inclusive.

## The Tuple type

It's a primitive compound type that is a general way of grouping a number of values with a variety of types.

It's fixed in length once declared.

[code sample](./tuples/src/main.rs)

## The Array type

Every array element must have the same type (unlike a tuple).

Arrays are fixed in length.

Arrays are useful if you want data allocated on the stack rather than the heap.

- The stack and the heap will be discussed in [Chapter 4.1 What is Ownership?](../../04-understanding-ownership/4-1-what-is-ownership/).

Useful to store, for example, the names of the months in a calendar year.

A vector is a similar collection type provided by the standard library that can grow and shrink in size.

- a vector is more flexible than an array
- vectors are discussed in [8.1. Storing Lists of Values with Vectors](../../08-common-collections/8-1-vectors/).

[code sample](./arrays/src/main.rs):

- see comment on why the length of an array has to be a `const` of type `usize`
