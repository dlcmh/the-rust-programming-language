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
