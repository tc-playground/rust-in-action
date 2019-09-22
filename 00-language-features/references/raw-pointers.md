# Pointers

## Introduction

> Rust `references` (and `smart pointers`) are ensured to be __valid__ by the Rust compiler.

* Rust `raw pointers` do not have the same guarantee. 

    * Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location.

    * Aren’t guaranteed to point to valid memory.

    * Are allowed to be null.

    * Don’t implement any automatic cleanup.

* They can be `mutable` or `immutable`.

* They can only be used in `unsafe mode` via the `unsafe` keyword.

---

## Example

```rust
let mut writer = Writer {
    column_position: 0,
    color_code: ColorCode::new(Color::Yellow, Color::Black),
    buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
};
writer.write_byte(b'X');
```

---

## Overview

* [Unsafe Rust](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html)

* [Dereferencing raw pointers](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#dereferencing-a-raw-pointer)

