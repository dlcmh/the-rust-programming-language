# 12. An I/O project: Building a command line program

A `grep` (globally search a regular expression and print) tool.

## Sample code

[minigrep before refactor](./01-before-refactor/minigrep/)

- `std::env::args`
  - returns an iterator of command line args
  - will `panic` if args contain invalid Unicode
  - use `std::env::args_os` - that returns an iterator over `OsString` to work with args that contain invalid Unicode

[minigrep after extracting argument parser](./02-after-refactor/01-extract-arg-parser/minigrep/)

[minigrep after `Config` struct](./02-after-refactor/02-config-struct/minigrep/)

[minigrep after constructor `new` on `Config` struct](./02-after-refactor/03-config-struct-constructor/minigrep/)

[minigrep after fixes to error handling](./02-after-refactor/04-fix-error-handling/minigrep/)

[minigrep after extracting run function](./02-after-refactor/05-extract-to-run/)

[minigrep after handling errors from run function](./02-after-refactor/05-extract-to-run/)
