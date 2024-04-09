
# Chapter 3

`x` is an expression. Adding a semi colon turns it into a void statement.

# Chapter 4 Ownership

Rust will 'move' variables rather than making shallow copies of them.

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s1);
}
```

`s1` stores a heap allocated `String`. It (the heap pointer) is then moved to 
`s2` which invalidates `s1` so this is no longer valid.

If `s1` were a string literal though, the heap wouldn't be involved and it would
work. This works for any type that implements the `Copy` trait.

---

Types can implement a `drop` function when they are freed to free their memory.

# Chapter 5

`dbg!()` prints its unevaluated expression, then evaluates it, then returns
the output + the line number from the source code.

# Chapter 7

Library crate = No `main()` function.

Binary crate = Has a `main()` function and can be compiled to an executable.

Packages can only contain one library create (crate with src/lib.rs)


`mod vegetables` to create a `vegetables module.
The code can then be placed inline (`mod vegetables {}`), in `vegetables.rs`, or in `vegetables/lib.rs`.

`pub mod` makes a module accessible to parent modules.

The idiomatic way to bring a function into scope is to use the `use` keyword but
only until the parent module.

This is preferred
```rust
use crate::front_of_house::hosting;

fn main() {
    hosting::add_to_waitlist();
}
```
Even though this is still valid
```rust
use crate::front_of_house::hosting::add_to_waitlist;

fn main() {
    add_to_waitlist();
}
```

But that is only true for functions. For Enums, structs etc, we use the full path.

`use std::io::{self, Write}` brings in `std::io` and `std::io::Write`.







