# 📁 Working with Files CLI

Just a simple project I'm working on to practice some of the basic operations of reading and writing files.

This is part of **Phase 1** of my journey learning **Rust** 🦀.

## 📝 Notes to Self

### .unwrap_or_else()
The `.unwrap_or_else()` method is super useful. Instead of handling both success and error cases with `match`, this method lets you focus only on the `None` or error scenario—without redundantly passing back an `Ok` result.

### usize
The `usize` type is particularly important when indexing collections or working with pointer arithmetic. It's recommended over using `i32` because:

- `usize` is an **unsigned integer** whose size matches the architecture of the system:
- On a 32-bit system, it's 32 bits.
- On a 64-bit system, it's 64 bits.
- This makes it ideal for **memory-safe indexing**, as it aligns with how pointers and memory offsets are represented internally.
- Using `i32` for indexing can lead to **truncation or overflow issues** on 64-bit systems, especially when dealing with large data structures.

🧪 Example:
```rust
let data = vec![10, 20, 30];
let index: usize = 1;
println!("{}", data[index]); // Safe and idiomatic

