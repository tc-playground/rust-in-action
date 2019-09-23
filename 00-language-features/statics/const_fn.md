# const_fn

## Introduction

* Allow the declaration onf `constant functions` in Rust.

---

## Example

```rust
#![feature(const_fn)]

const fn double(x: i32) -> i32 {
    x * 2
}

const FIVE: i32 = 5;
const TEN: i32 = double(FIVE);

fn main() {
    assert_eq!(5, FIVE);
    assert_eq!(10, TEN);
}
```

---

## References

* [`const_fn`](https://doc.rust-lang.org/unstable-book/language-features/const-fn.html)