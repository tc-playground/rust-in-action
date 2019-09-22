# UnsafeCell

## Introduction

* `UnsafeCell<T> `is a type that wraps some T and indicates unsafe interior operations on the wrapped type. 

* Types with an `UnsafeCell<T>` field are considered to have an 'unsafe interior'. 

* The `UnsafeCell<T>` type is the only legal way to obtain aliasable data that is considered mutable. 

* In general, transmuting an `&T` type into an `&mut T` is considered __undefined behavior__.

* `UnsafeCell<T>` is the only core language feature to work around the restriction that &T may not be mutated. 

* Types that allow internal mutability, such as `Cell<T>` and `RefCell<T>`, use `UnsafeCell` to wrap their internal data. 

* There is no legal way to obtain aliasing &mut, not even with `UnsafeCell<T>`.

> `UnsafeCell` is not `Sync`, so we can't be used with statics (unless manually synchronised).

---

## References

* [UnsafeCell - Rust Docs](https://doc.rust-lang.org/nightly/core/cell/struct.UnsafeCell.html)