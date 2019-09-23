# Dynamic Borrowing

## Introduction

* `Dynamic borrowing` is a process whereby one can claim temporary, exclusive, mutable access to the inner value. 

* `RefCell<T>` uses Rust's lifetimes to implement `dynamic borrowing`.

* Borrows for `RefCell<T>`s are tracked at __runtime__, unlike Rust's native reference types which are entirely tracked statically, at compile time.

* Because `RefCell<T>` borrows are __dynamic__ it is possible to attempt to borrow a value that is already mutably borrowed; when this happens it results in thread panic.

---

## References

* [Cell - Rust Docs](https://doc.rust-lang.org/nightly/core/cell/index.html)