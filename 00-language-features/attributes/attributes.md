# Rust Attributes

## Introduction

In Rust `Attributes` are bits of metadata that can be added `programming constructs` (`compiler`, `modules`, `declarations`, `statements`, `blocks`, `enums`, etc) to define their behaviour.

---

## Examples

* __allow__ - Controls compiler behaviour.

    * `#[allow(dead_code)]` - Prevents compiler errors for unused members.

* __derive__ - Derive features from other `Traits`.

    * `#[derive(Debug, Clone, Copy, PartialEq, Eq)]` - Defines `copy semantics` for the elements.

* __[repr](https://doc.rust-lang.org/nomicon/other-reprs.html#reprtransparent)__ - Define how an entity (e.g. `enum`/`struct`) is defined.

    * `#[repr(u8)]` - Specify the size to make a fieldless enum. In this case `8 bits`.

    * `#[repr(C)]` - The layout and ABI of the whole struct is guaranteed to be the same  a `C struct` to ensure the correct field ordering.

    * `#[repr(transparent)]` - The layout and ABI of the whole struct is guaranteed to be the same as that one field.

---

## References

* [Attributes - Rust Reference](https://doc.rust-lang.org/reference/attributes.html)

* [Attributes - Rust by Example](https://doc.rust-lang.org/rust-by-example/attribute.html)