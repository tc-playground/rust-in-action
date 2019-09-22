# RefCell

## Introduction

* A mutable memory location with dynamically checked borrow rules.

* `Interior mutability` is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data.

    * Normally, this action is disallowed by the borrowing rules.

* To mutate data, the pattern uses `unsafe` code inside a data structure to bend Rust’s usual rules that govern `mutation` and `borrowing`.

* We can use types that use the interior mutability pattern when we can ensure that the borrowing rules will be followed at runtime, even though the compiler can’t guarantee that.

* `RefCell<T>` type represents single ownership over the data it holds.

* With `references` and `Box<T>`, the borrowing rules’ invariants are __enforced at compile time__. With `RefCell<T>`, these invariants are __enforced at runtime__. 

    * With `references`, if you break these rules, you’ll get a __compiler error__. 
    
    * With `RefCell<T>`, if you break these rules, your program will __panic and exit__.


* `RefCell<T>` keeps track of how many Ref<T> and RefMut<T> smart pointers are currently active at `runtime`.

> `RefCell` is not `Sync`, so we can't be used with statics (unless manually synchronised).

---

## References

* [Interior Mutability - Rust Book](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html)

* [RefCell - Rust Docs](https://doc.rust-lang.org/nightly/core/cell/struct.RefCell.html)

* [Cell - Rust Docs](https://doc.rust-lang.org/nightly/core/cell/index.html)