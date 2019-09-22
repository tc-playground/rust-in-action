# std::cell::Ref

## Introduction

* Wraps a borrowed reference to a value in a `RefCell` box. 

* A wrapper type for an immutably borrowed value from a `RefCell<T>`.

* Returned from `RefCell<T>#borrow()`.

> The `RefCell<T>` keeps track of how many `Ref<T>` and `RefMut<T>` smart pointers are currently active at `runtime` and panics if the borrowing rules are violated.

---

## References

* [Struct std::cell::Ref - Rust Docs](https://doc.rust-lang.org/std/cell/struct.Ref.html)
