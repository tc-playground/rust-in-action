# Structs

## Introduction

1. `Rust` supports the concept of `C like structs`.

2. `Attributes` can describe the implementation.

---

## Example

```rust
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}
```

* __Attributes__

    * `#[allow(dead_code)]` - Prevents compiler errors for unused members.

    * `#[derive(Debug, Clone, Copy, PartialEq, Eq)]` - Defines `copy semantics` for the elements.

    * `#[repr(u8)]` - Represents instances of the struct using 8 bits.

    * `#[repr(C)]` - Represent instance of the struct as a `C struct`. This preserves the ordering of the bits.

---

## References

* [`C-like enums`](https://doc.rust-lang.org/rust-by-example/custom_types/enum/c_like.html)