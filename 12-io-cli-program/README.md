# 12. An I/O project: Building a command line program

A `grep` (globally search a regular expression and print) tool.

## Sample code

[minigrep](./minigrep/)

- `std::env::args`
  - returns an iterator of command line args
  - will `panic` if args contain invalid Unicode
  - use `std::env::args_os` - that returns an iterator over `OsString` to work with args that contain invalid Unicode
