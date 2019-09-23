# `new type` Idio

## Introduction

The `newtype` idiom gives compile time guarantees that the right type of value is supplied to a program.

---

## Example

* Define a `Year` as a type:

    ```rust
    struct Years(i64);
    struct Days(i64);
    ```

---

## References

* [New Type Idiom - Rust by Example](https://doc.rust-lang.org/rust-by-example/generics/new_types.html)