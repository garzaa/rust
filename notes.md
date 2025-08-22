# Chapter 2
- you need to explicitly specify mut, so a bit like JS/TS
- `let` creates a variable but it's `const` by default
- `&` is a reference, references are immutable by default, therefore just use `&mut` to signal something is a _mutable reference_
- there's a lot of cool syntax for range expressions
- println! is an exclamation mark because it's a macro for doing a lot of typesafe conversion stuff
- *shadowing* is new, it lets you overwrite a previous variable with a new one (e.g. to change the type without making
extra variable names)
  - it is *NOT* like `mut` because we create a new variable with the same name, overwriting it (but only temporarily if in a lower scope)
  - you can shadow a `mut` variable if you use the `let` keyword, and do it with the same type/different val if you also use `let`

## Results
- Enums, `Ok` and `Err` (rather than a return, err like Go)
- not using `expect` on a `Result` type will compile but with errors



# Chapter 3: Variables
- the `mut` keyword is used to let variables be changed (via `=`)
- constants are made with the `const` keyword, only available in global scope (makes sense)
- you can *shadow* variables inside scopes, and it's only applied inside the scope
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
- variables live in *frames*, which are mappings from variable -> value in a single scope, and on the stack of called functions
- you can put basically anything (?) in a *box* to put data on the heap, e.g.:
```rust
    let a = Box::new([0:1_000_000]);
    let b = a; // saves memory by not copying a
```
- `a` has now been *moved* (it doesn't own the box)
- you can't manually free things
- when a heap item's *owner's* frame goes out of scope or is otherwise deallocated, the heap item is also freed
  - in this case, b is the owner of the box defined originally by a, and the box is tied to b now
- collections use boxes, stuff like vec, string, hashmaps, etc
- you can't use a variable after it's been moved (`a` up above)
- you can get around this with `clone()` if you really need a copy. it copies not pointers (like a pointer to a string on the heap) but clones the heap value itself
- all heap data is owned by exactly ONE variable
- once its owner leaves scope, the data is deallocated
- ownership can only be transferred by *moves*
  - these are implicit and happen on assignments and function calls
- heap data has to be accessed by its current owner

## 4.2 References & Borrowing
- to use data in a function without an actual move, use a reference
- the & operator borrows it, it becomes a type of `&String` or a reference to a string
- references are *non-owning* pointers
- to follow a reference, use `*` ,the *dereference* operator
```rust
    let mut x = Box::new(1);
    let r1 = &x;
    let b = *r1;
    println!("{b}"); // 1
    // adding 1 with funny stuff
    let mut c = b;
    c += 1;
    let r2 = &c;
    println!("{}", *r2); // 2
```
- there are explicit and implicit dereferences, most of the time it's implicit with the dot operator (you can just call .abs() on  a Box<i32>)
- it also works in reverse with implicit references. like if you call len() on an owned String (instead of an expected &str) then Rust will automatically insert a reference
- pointer safety: data should never be aliased AND mutated
- borrow checker gives variables 3 kinds of permisisons:
  READ, WRITE, OWN
- `mut` is what gives a variable WRITE on its own data
- _references temporarily remove these permissions_
- permissions are defined on *paths*, not variables
- *paths* are anything you can put on the left of an assignment, like:
  - variables
  - dereferences of paths
  - array accesses of paths
  - fields of paths (a.0 for tuples and a.field for structs)
paths lose permissions when they become unused
- however, there are also *mutable references* (aka *unique references*), which provide unique and non-owning access to data
```rust
let mut v: Vec<i32> = vec![1, 2, 3];
let num: &mut i32 = &mut v[2];
```
- mutable references prevent even reading from what they reference, to avoid memory errors
- so here, `v` will be completely unusable while `num` is active
- you can also downgrade a mutable reference to an immutable one by shadowing it
- data needs to outlive any references to it, which makes sense (you can't call `drop` on something you don't own)
- so now we need a new type of permission: *flow*

### Flow
- expected whenever an expression uses an "input reference" (a reference to something as an input, like `&strings[0]`)
- OR whenever it returns an output reference (like `return s_ref`)
- a reference has the Flow permission if it's allowed to, yes, *flow*, through a particular expression
- inputs and outputs to functions have to match to each other with the borrow checker
- e.g. you can't branch and return a reference to one or another
- those things are called *lifetime parameters* for later chapters

## 4.3 Fixing Ownership Errors
- one way to get around reference passing is by passing mutable references in fn args
- however, *functions should NOT mutate things unless the caller expects it*
- it is also rare for Rust functions to take ownership of heap objects like Vec and String (it would make the inputs unusable)
- if, for example, you're using a function to append something to a string: clone it somehow, modify the clone, then return that
- generally, only ask for as much permissions as you _absolutely_ need
- pay attention to function names and whether things are expected to modify something in place or add it

bookmark:
https://rust-book.cs.brown.edu/ch04-03-fixing-ownership-errors.html#fixing-an-unsafe-program-aliasing-and-mutating-a-data-structure

- what happens if you want to copy a value out of an array? and then dereference it?
- strings it'll complain, because the array owns the string in it.
- DEREFERENCING TAKES OWNERSHIP
- and you can't do that, it'll be unsafe. both things think they own that string
- when one leaves scope or whatever, it'll remove it, and the other will be pointing to...undefined
- why ints and not strings? strings contain heap data, and so they can't be moved.
- stack data can be copied because it doesn't own heap data, like an i32
- mutable references (like `&mut i32` are not copyable types)
- so how do you do it? either
  - use an immutable reference (doesn't take ownership)
  - clone it if you want ownership
  - use `vec::remove` to move it
- also: Rust doesn't look at function implementations when borrow checking. it's opaque
- either inline it or use cells...?
- rust also treats any array index as all of them, beacuse it can vary

## 4.3 Slice Type
- they're references to a continuous sequence of elements in an array
- non owning pointer
- returning indices might be invalid if the string changes in the future
- instead, they use notations
```rust
let s = String::from("hello world");

let hello: &str = &s[0..5];
let world: &str = &s[6..11];
```
- ^^^ those are slices
- slices are "Fat" pointers, because they have metadata (length of slice)
- because slices are references, they change permissions on referenced data
- string literals are slices - they point to a specific part of the compiled binary

Quiz recap:

Say you are writing a function with the following spec:

    find_contains takes as input a collection of strings and a target substring. It returns a list of all the strings in the collection that contain the target substring.

Context: For `haystack`, the slice type `&[String]` can accept more inputs than `&Vec<String>`, so it is preferred. For `needle`, the target substring does not need to be heap-allocated, so `&str` is preferred to `String`. For the return type, `Vec<String>` is not desirable because it would require cloning the input strings. `&[String]` is not desirable because it can only return a contiguous subsequence of the input. `Vec<&String>` is the most preferable because it only incurs the cost of allocating the vector, not the strings themselves.

# Chapter 5: Structs
## Creating
- they're more similar to Go structs
- You can use field init shorthand to not do the circus of a = new() { a.name = name; a.number = number}.
- struct update shorthand lets you specify the changed values first and then do ...templateStruct to do the rest
- you can also do Unit-like structs for if you want to implement a trait on a type but don't need to store data in it
- structs usually own their own data, but if they need to reference external data, you need to worry about `lifetimes`

bookmark: https://rust-book.cs.brown.edu/ch05-02-example-structs.html

- when doing struct methods you usually want `&self`
- `&self` doesn't take ownership and just reads data without writing
- `&mut self` returns, you guessed it, mutable reference, so you can write to self data
- `self` is rare, it's for something like transforming the struct into something else, something that needs ownership
- you can have associated functions (in an `impl` block) that aren't methods, used for constructors mainly
- you can have multiple `impl` blocks for structs
- method calls are syntactic sugar for function calls - Rust automatically inserts `&`
- also, no `a.b()` and `a->b`, Rust automatically references/dereferences them
if r is a Rectangle struct:
```rs
    r.set_width(2);
    Rectangle::set_width(&mut r, 2);
```
- `r.set_width(2)` becomes `Rectangle::set_width(&mut r, 2)`
- Rust will also unbox things and add the dereference operators
- there's actually no constructor keyword, the idiomatic way is to implement a `new()` function
-  
