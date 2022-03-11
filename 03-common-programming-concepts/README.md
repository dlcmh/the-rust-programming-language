# Common Programming Concepts

## Variables

By default, variables are immutable.

```rust
let w = 5; // create immutable variable y and bind y to a value of 5
let mut x = 5; // create mutable variable x and bind x to a value of 5
```

## Constants

Unlike variables, constants:

- don't allow the use of `mut`
- can be declated in any scope, including the global scope
- must be annotated with the type of value

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

## Shadowing of variables

Shadowing is done by:

- using the same variable name
- repeating the use of the `let` keyword

```rust
let x = 5; // (A) - binds x to 5

// (B)
//   - shadows x by repeating `let x =`
//   - x becomes 6 at the completion of the evaluation of this expression
let x = x + 1;

// The variable `x` in (A) is shadowed by `x` in (B).

// begins an inner scope
{
  let x = x * 2; // (C) x is 12
}

// (D) value of x remains at 6
```

Shadowing allows the reuse of a variable's name while performing transformations on a value.

Transformations can change the type of the value because a new variable gets effectively created with the use of the `let` keyword.

```rust
let spaces = "     "; // string type
let spaces = spaces.len(); // number type
```

The variable remains immutable after transformations.

Changing the value type of a shadowed mutable variable without use of the `let` keyword isn't allowed.

```rust
let mut x = 10; // creates mutable variable x and binds it to 5
x = 20; // allowed
x = "30"; // not allowed
let x = "30"; // allowed; also creates a new immutable variable named `x` with type of string
```
