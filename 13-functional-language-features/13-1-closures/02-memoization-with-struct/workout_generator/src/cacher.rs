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
    calculation: T,

    //   The following logic is defined in `impl<T> Cacher<T>`:
    //   - before the closure is executed, `value` will be `None`
    //   - when code using a `Cacher` askd for the result of the closure, the `Cacher`
    //     will execute the closure and store the result within a `Some` variant in the
    //     `value` field
    //   - if the code asks for the result of the clousure again, instead of executing the
    //     closure again, the `Cacher` will return the result held in the `Some` variant
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    // takes a generic parameter `T`, which is defined as having the same trait bound
    // as the `Cacher` struct
    pub fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    pub fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
