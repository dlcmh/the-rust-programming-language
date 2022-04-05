pub struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    // trait bounds on `T` specify that `calculation` field is a closure by using the `Fn` trait
    pub calculation: T,
    pub value: Option<u32>,
}
