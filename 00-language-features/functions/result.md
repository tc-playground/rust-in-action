# Result


* `Result` types are a mechanism for dealing with unexpected result return from function calls.

---

## Example - `unwrap` a `write` macro Result.

```rust
use core::fmt;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.my_write_string(s);
        Ok(())
    }
}

...

write!(writer, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();
```