# Mutable Statics

## Introduction

1. `statics` are Rust's equivalent of `globals`.

2. `mutable statics` can be problematic with Rust’s `ownership` rules. 

    1. If two threads are accessing the same `mutable global variable`, it can cause a `data race`.

3. `mutable statics` are generally to be avoided. Instead, `interior mutability` should be sought. 

4. The Rust `RefCell` pattern and `Unsafe Cell` primitive provide `interior mutability`.

---

## References

* [Accessing or Modifying Mutable Static Variables](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#accessing-or-modifying-a-mutable-static-variable)

* [Interior Mutability](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html)

* [Unsafe Cell API](https://doc.rust-lang.org/nightly/core/cell/struct.UnsafeCell.html) - Interior mutability primitive.