# std::sync::RwLock

## Introduction

* A `reader-writer` lock.

* `RwLock` allows any number of `readers` or at most one `writer` at any point in time.

* `RwLock` will allow _any number of readers_ to acquire the lock as long as _a writer is not holding the lock_.

* The priority policy of the lock is dependent on the underlying operating system's implementation.

* `RwLock` implements `poisoning`.

    * A RwLock is considered `poisoned` whenever a thread panics while holding the mutex. 
    
    * Once a RwLock is `poisoned`, all other threads are unable to access the data by default.

> NB: `Mutex` does not distinguish between `readers` or `writers` that acquire the lock.

---

## References

* [std::sync::RwLock - Rust Docs](https://doc.rust-lang.org/nightly/std/sync/struct.RwLock.html)