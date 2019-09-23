# Traits

## Introduction

* `Traits` are a mechanism in Rust for defining and implementing behaviour.

---

## Example - Implement `core::fmt:Write` Trait

```rust
use core::fmt;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.my_write_string(s);
        Ok(())
    }
}
```

