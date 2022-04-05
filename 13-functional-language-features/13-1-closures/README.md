# 13.1. Closures: Anonymous functions that can capture their environment

Closures:

- can be saved in a variable
- can be passed as arguments to functions
- can be created in one place, and then later called to evaluate it in a different context
- can capture values from the scope in which they're defined (unlike functions)
- allow for code reuse
- allow for behavior customisation
- need to be called at least once for the compiler to infer the types of their parameters & return value, if no arguments from the outer scope are passed into the definition

## Sample code

[Naively call expensive function multiple times as needed](./01-abstract-behavior/workout_generator/src/workout.rs)

[Lazy evaluation / lazily evaluate / memoization / memo / cache / caching implementation with a `Cacher` struct typed with a `Fn` trait bound that closures must match](./02-memoization-with-struct/workout_generator/src/)
