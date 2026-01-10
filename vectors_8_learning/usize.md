
```rust
let i: usize = 1;
println!("{}", i); // prints 1

```

> `usize` is the right size integer to index memory, not store memory addresses.

**If something represents a count or index, Rust uses usize.**

```rust
len() -> usize
capacity() -> usize
position() -> Option<usize>
enumerate() -> (usize, T)
```
