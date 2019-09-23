# core::sync::atomic

## Introduction

* `Atomic types` provide primitive shared-memory communication between threads, and are the building blocks of other concurrent types.

* `Atomic types` present operations that, when used correctly, synchronize updates between threads.

    * `AtomicBool`, `AtomicIsize`, `AtomicUsize`, `AtomicI8`, `AtomicU16`.

* `Atomic types` in this module are guaranteed to be `lock-free` if they're available.

* `Atomic types` and operations are not guaranteed to be `wait-free`.

---

## References

* [core::sync::atomic - Rust Docs](https://doc.rust-lang.org/nightly/core/sync/atomic/index.html)
