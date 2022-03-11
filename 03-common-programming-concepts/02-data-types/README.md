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

The stack and the heap will be discussed in [Chapter 4. Understanding Ownership](../../04-understanding-ownership/).
