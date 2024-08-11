https://rust-book.cs.brown.edu/ch04-02-references-and-borrowing.html

# Chapter 2
- you need to explicitly specify mut, so a bit like JS/TS
- `let` creates a variable but it's `const` by default
- `&` is a reference, references are immutable by default, therefore just use `&mut` to signal something is a _mutable reference_
- there's a lot of cool syntax for range expressions
- println! is an exclamation mark because it's a macro for doing a lot of typesafe conversion stuff
- **shadowing** is new, it lets you overwrite a previous variable with a new one (e.g. to change the type without making
extra variable names)
  - it is **NOT** like `mut` because we create a new variable with the same name, overwriting it (but only temporarily if in a lower scope)
  - you can shadow a `mut` variable if you use the `let` keyword, and do it with the same type/different val if you also use `let`

## Results
- Enums, `Ok` and `Err` (rather than a return, err like Go)
- not using `expect` on a `Result` type will compile but with errors



# Chapter 3: Variables
- the `mut` keyword is used to let variables be changed (via `=`)
- constants are made with the `const` keyword, only available in global scope (makes sense)
- you can **shadow** variables inside scopes, and it's only applied inside the scope
- rust has int types from 8 to 128 (8, 32, 64) and defaults to 32
- also two float types, 32 and 64
- you can do emoji characters
- tuples are ordered types, all gotta be the same
- arrays are a thing, they can't change length (use vectors for that)
- the syntax `[x; y]` declares an array with y copies of the value x! very cool
- anything in the last line of an expression {} without a semicolon is an implicit return
- you can do `let x = if condition {5} else {6}` (one line tertiary, types have to match (they're arms))
- there is no truthy or falsy things, it has to be hard bools
- you can name loops to break at different levels
- you can do python-style loops `for x in a` where a is some array of ints or something

# Chapter 4: Ownership
- memory safety guaranteees without a GC
- "safety is the absence of undefined behavior"
- ownership is checked at compile time
- variables live in **frames**, which are mappings from variable -> value in a single scope, and on the stack of called functions
- you can put basically anything (?) in a **box** to put data on the heap, e.g.:
```rust
    let a = Box::new([0:1_000_000]);
    let b = a; // saves memory by not copying a
```
- `a` has now been **moved** (it doesn't own the box)
- you can't manually free things
- when a heap item's **owner's** frame goes out of scope or is otherwise deallocated, the heap item is also freed
  - in this case, b is the owner of the box defined originally by a, and the box is tied to b now
- collections use boxes, stuff like vec, string, hashmaps, etc
- you can't use a variable after it's been moved (`a` up above)
- you can get around this with `clone()` if you really need a copy. it copies not pointers (like a pointer to a string on the heap) but clones the heap value itself
- all heap data is owned by exactly ONE variable
- once its owner leaves scope, the data is deallocated
- ownership can only be transferred by **moves**
  - these are implicit and happen on assignments and function calls
- heap data has to be accessed by its current owner
