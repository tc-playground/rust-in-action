# Volatile

## Introduction

* Aggressive compiler optimisations may check for variable `writes` without corresponding `reads` and decide to optimise out the write by not performing it.

* In other languages the compiler can be instructed not to do this using the `volatile` keyword.

* In Rust this feature is implemented in the core library and as a `crate`.

    * Provides a `Volatile` wrapper type with `read` and `write` methods. 
    
    * These methods internally use the `read_volatile` and `write_volatile` functions of the `core` library.

> This may be useful when accessing `write-only memory` e.g. `memory mapped io`, `VGA memory`.

---

## Example

* Convert a `ScreenBuffer` to a `Volatile<ScreenBuffer>`

    1. Original

        ```rust
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[repr(C)]
        struct ScreenChar {
            ascii_character: u8,
            color_code: ColorCode,
        }

        const BUFFER_HEIGHT: usize = 25;
        const BUFFER_WIDTH: usize = 80;

        #[repr(transparent)]
        struct Buffer {
            chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
        }rs: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
            }
        ```

    2. Dependency `cargo.toml`

        ```toml
        [dependencies]
        volatile = "0.2.3"
        ```

    3. Volatile Version

        ```rust
        use volatile::Volatile;

        struct Buffer {
            chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
        }
        ```

---

## References

* [Volatile - Wikipedia](https://en.wikipedia.org/wiki/Volatile_(computer_programming))



