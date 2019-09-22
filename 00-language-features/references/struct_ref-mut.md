# std::cell::RefMut

## Introduction

* Wraps a mutable borrowed reference to a value in a `RefCell<T>` box. 

* A wrapper type for a mutably borrowed value from a `RefCell<T>`.

* Returned from `RefCell<T>#borrow_mut()`.

> The `RefCell<T>` keeps track of how many `Ref<T>` and `RefMut<T>` smart pointers are currently active at `runtime` and panics if the borrowing rules are violated.

---

## References

* [Struct std::cell::RefMut - Rust Docs](https://doc.rust-lang.org/std/cell/struct.RefMut.html)
