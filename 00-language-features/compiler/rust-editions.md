# Rust Editions

## Overview

1. There are two `editions` of Rust:

    1. `2015 Edition`

        > NB: Requires `extern crate` statement before using anything outside of `std`. 

    2. `2018 Edition`

2. The edition can be specified by a flag when compiling, e.g. `rustc --edition=2018`

3. The edition can be specified in the `Cargo.toml` configuration file:

    ```toml
    [package]
    edition = "2018"
    ```

---

## References

* [Rust Editions Reference](https://doc.rust-lang.org/nightly/edition-guide/introduction.html)

