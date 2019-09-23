# Copy Semantics

## Introductions

`Copy semantics` are the set of `traits` that are required to be able to copy and compare structures.

---

## Example

* Define a `copyable` ColourCode

    ```rust
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    struct ColourCode(u8);

    impl ColourCode {
        fn new(foreground: Colour, background: Colour) -> ColourCode {
            ColourCode((background as u8) << 4 | (foreground as u8))
        }
    }
    ```

---

## References

* [Ownership - Copying](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)