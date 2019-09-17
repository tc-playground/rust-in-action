# `unsafe` Keyword

## Introduction

---

## Overview

* To switch to unsafe Rust, use the unsafe keyword and then start a new block that holds the unsafe code. 

    ```rust
    unsafe {
        // do stuff
    }
    ```

* You can take `four actions` in unsafe Rust, called `unsafe superpowers`:

    1. Dereference a raw pointer.
    
    2. Call an unsafe function or method
    
    3. Access or modify a mutable static variable.
    
    4. Implement an unsafe trait.

* `unsafe` doesn’t turn off the borrow checker or disable any other of Rust’s safety checks.


---

## References

* [Unsafe Rust](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html)

* [Rust `unsafe` Docs](https://doc.rust-lang.org/stable/book/ch19-01-unsafe-rust.html#unsafe-superpowers)