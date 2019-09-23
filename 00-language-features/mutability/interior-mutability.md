# Interior Mutability

## Introduction

1. `Interior mutability` is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data.

    * NB: Normally, this action is disallowed by the borrowing rules.

2. To mutate data, the pattern uses unsafe code inside a data structure to bend Rust’s usual rules that govern mutation and borrowing.

> `mutable statics` are generally to be avoided. Instead, `interior mutability` should be sought. 

> The Rust `RefCell` pattern and `Unsafe Cell` primitive provide `interior mutability`.

---

## References

* [Accessing or Modifying Mutable Static Variables](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#accessing-or-modifying-a-mutable-static-variable)

* [Interior Mutability](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html)

* [Unsafe Cell API](https://doc.rust-lang.org/nightly/core/cell/struct.UnsafeCell.html) - Interior mutability primitive.
