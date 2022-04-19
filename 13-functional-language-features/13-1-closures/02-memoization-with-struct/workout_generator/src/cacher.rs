// `Cacher` struct:
// - holds a closure in `calculation` field
// - holds an optional result, `Option<u32>`, in the `value` field
pub struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    //   - `calculation` field is of the generic type `T`
    //   - trait bounds on `T` specify that it's a closure by using the `Fn` trait
    //   - closure stored on `calculation` field must have on `u32` parameter and
    //     must return a `u32`
    pub calculation: T,

    //   - before the closure is executed, `value` will be `None`
    //   - when code using a `Cacher` askd for the result of the closure, the `Cacher`
    //     will execute the closure and store the result within a `Some` variant in the
    //     `value` field
    //   - if the code asks for the result of the clousure again, instead of executing the
    //     closure again, the `Cacher` will return the result held in the `Some` variant
    pub value: Option<u32>,
}
