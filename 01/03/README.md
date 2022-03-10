# Hello, Cargo!

`cargo check` - manually run this periodically to ensure code can compile without producing an executable

`cargo build --release`:

- compiles with optimizations
- creates executable in "target/release" instead of "target/debug"

## Working on an existing project

```sh
git clone example.org/someproject
cd someproject
cargo build
```
