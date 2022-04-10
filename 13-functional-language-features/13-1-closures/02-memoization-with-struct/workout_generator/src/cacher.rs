// `Cacher` struct:
// - holds a closure in `calculation` field
//   - `calculation` field is of the generic type `T`
//   - trait bounds on `T` specify that it's a closure by using the `Fn` trait
//   - closure stored on `calculation` field must have on `u32` parameter and
//     must return a `u32`
// - holds an optional result in `value` field
pub struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    pub calculation: T,
    pub value: Option<u32>,
}
