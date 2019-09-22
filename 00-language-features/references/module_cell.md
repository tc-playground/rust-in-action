# Module core::cell

## Introduction

1. Rust memory safety is based on this rule: Given an object T, it is only possible to have one of the following:

    1. __Aliasing__ - Having several immutable references (&T) to the object (also known as `aliasing`).
    
    2. __Mutability__ - Having one mutable reference (&mut T) to the object (also known as `mutability`).

2. This is enforced by the Rust compiler.

3. There are situations where this rule is not flexible enough. 

    1. __Multiple Reference to Mutable Object__ - Sometimes it is required to have __multiple references to an object and yet mutate it__.

4. Shareable mutable containers exist to permit mutability in a controlled manner, even in the presence of aliasing. 

5. Both `Cell<T>` and `RefCell<T>` allow doing this in a __single-threaded__ way. 

6. Neither `Cell<T>` nor `RefCell<T>` are __thread safe__ (they do not implement `Sync`). 

    1. If you need to do aliasing and mutation between multiple threads it is possible to use `Mutex`, `RwLock` or `atomic` types.

---

* __Interior Mutability__ - Values of the Cell<T> and RefCell<T> types may be mutated through shared references (i.e. the common &T type)

* __Inherited Mutability__ - Most Rust types can only be mutated through unique (&mut T) references. 

---

## References

* [Module core::cell - Rust Docs](https://doc.rust-lang.org/nightly/core/cell/index.html)