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

**`new syntax (A)`** defines an [associated type](../../19-advanced-features/19-2-advanced-traits/README.md/#specifying-placeholder-types-in-trait-definitions-with-associated-types) with this trait:

- implementing the `Iterator` trait requires an `Item` type to be defined
- the `Item` type is used in the return type of the `next` method
- the `Item` type will be the type returned from the iterator

Implementors of the `Iterator` trait must define one method, the `next` method:

- which returns one item of the iterator at a time wrapped in `Some`,
- and, when the iteration is over, returns `None`

The `next` method can be called on iterators directly:

```rust
#[test]
fn iterator_demonstration() {
  let v1 = vec![1, 2, 3];

  // Note (A)
  // v1_iter must be mutable, otherwise error on `v1_iter.next()`:
  //   cannot borrow `v1_iter` as mutable, as it is not declared as mutable
  let mut v1_iter = v1.iter();

  // Note (B)
  // `Some(&1)`, otherwise error:
  //   expected `&{integer}`, found integer
  assert_eq!(v1_iter.next(), Some(&1));
  assert_eq!(v1_iter.next(), Some(&2));
  assert_eq!(v1_iter.next(), Some(&3));
  assert_eq!(v1_iter.next(), None);

  // Note (C) `into_iter`
  let v1 = vec![1, 2, 3];
  let mut v1_iter = v1.into_iter();
  assert_eq!(v1_iter.next(), Some(1));
  assert_eq!(v1_iter.next(), Some(2));
  assert_eq!(v1_iter.next(), Some(3));
  assert_eq!(v1_iter.next(), None);

  // Note (D) `iter_mut`
  let mut v1 = vec![1, 2, 3];
  let mut v1_iter = v1.iter_mut();
  assert_eq!(v1_iter.next(), Some(&mut 1));
  assert_eq!(v1_iter.next(), Some(&mut 2));
  assert_eq!(v1_iter.next(), Some(&mut 3));
  assert_eq!(v1_iter.next(), None);
}
```

**Note (A)**: Iterator `v1_iter` needs to be made mutable:

- calling the `next` method changes the internal state used by the iterator:
  - the internal state is used to keep track of where the iterator is in the sequence
- calling `next`:
  - consumes, or uses up, the iterator
  - eats up an iterm from the iterator
- `v1_iter` needn't be made mutable when used in a `for` loop because:
  - the loop takes ownership of `v1_iter` and makes it mutable behind the scenes

**Note (B)**: Values obtained from calls to `next`:

- are immutable references to the values in the vector `v1`
- the `iter` method in `v1.iter()` produces an iterator over immutable references:
  - **Note (C)**: to create an iterator that takes ownership of `v1` and returns owned values, use `into_iter`
  - **Note (D)**: to create an iterator over mutable references, use `iter_mut`
