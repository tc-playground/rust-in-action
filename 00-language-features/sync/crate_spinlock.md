# Spin Locks

## Introduction

* `spinlocks` are a really basic kind of mutex in computer science that requires no operating system features.

* Instead of blocking, the threads simply try to lock it again and again in a tight loop .

    * i.e. burn CPU time until the mutex is free again.

* Safe for #[no_std] environments.

---

## References

* [`Spin Lock` - Wikipedia](https://en.wikipedia.org/wiki/Spinlock)

* [spin Crate](https://crates.io/crates/spin)

* [spin Github](https://github.com/mvdnes/spin-rs)