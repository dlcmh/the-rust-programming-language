# Processing a series of items with iterators

- iterator pattern is code that performs some task on a sequence of items in turn
- an iterator is responsible for:
  - the logic of iterating over each item
  - determining when a sequence is finished
- an iterator is lazy:
  - it has no effect until...
  - ...the code calls methods that consume the iterator to use it up

```rust
let v1 = vec![1, 2, 3]; // vector `v1`

// - call `iter` method defined on `Vec<T>`
//   -> creates iterator `v1_iter` over items in vector `v1`
// - at this point, the code does not do anything useful;
//   - remember that an iterator is lazy
let v1_iter = v1.iter();

// use `v1_iter` with a `for` loop to run code on each item on every iteration
for val in v1_iter {
  println!("Got: {}", val);
}
```

## The `Iterator` trait and the `next` method

Iterators implement a trait named `Iterator` that is defined in the standard library:

```rust
pub trait Iterator {
  // new syntax (A): `type Item`
  type Item;

  // new syntax (A): `Self::Item`
  fn next(&mut self) -> Option<Self::Item>;

  // methods with default implementations elided
}
```

`new syntax (A)` defines an [associated type](../../19-advanced-features/19-2-advanced-traits/README.md/#specifying-placeholder-types-in-trait-definitions-with-associated-types) with this trait:

- implementing the `Iterator` trait requires an `Item` type to be defined
- the `Item` type is used in the return type of the `next` method
- the `Item` type will be the type returned from the iterator
