
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


