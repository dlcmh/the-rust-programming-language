fn main() {
    let v = vec![1, 2, 3];

    // "buffer overread" undefined behavior in C
    println!("{}", v[99]);
    // thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:5:20
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

    // with `RUST_BACKTRACE=1 cargo run`
    // thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:5:20
    // stack backtrace:
    //    0: rust_begin_unwind
    //              at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/std/src/panicking.rs:498:5
    //    1: core::panicking::panic_fmt
    //              at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/panicking.rs:116:14
    //    2: core::panicking::panic_bounds_check
    //              at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/panicking.rs:84:5
    //    3: <usize as core::slice::index::SliceIndex<[T]>>::index
    //              at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/slice/index.rs:189:10
    //    4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
    //              at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/slice/index.rs:15:9
    //    5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
    //              at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/alloc/src/vec/mod.rs:2520:9
    //    6: backtrace::main
    //              at ./src/main.rs:5:20
    //    7: core::ops::function::FnOnce::call_once
    //              at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a/library/core/src/ops/function.rs:227:5
    // note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
}
