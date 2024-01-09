https://rust-book.cs.brown.edu/ch01-03-hello-cargo.html

# Chapter 2
- you need to explicitly specify mut, so a bit like JS/TS
- `let` creates a variable but it's `const` by default
- `&` is a reference, references are immutable by default, therefore just use `&mut` to signal something is a _mutable reference_

## Results
- Enums, `Ok` and `Err` (rather than a return, err like Go)
- not using `expect` on a `Result` type will compile but with errors
