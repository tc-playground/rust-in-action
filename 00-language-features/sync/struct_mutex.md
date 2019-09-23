# std::sync::Mutex

## Introduction

* A `mutual exclusion primitive` useful for protecting shared data.

* `Mutex` will block threads waiting for the lock to become available.

* `Mutex` can be statically initialized or created via a `new` constructor. 

* `Mutex` has a type parameter which represents the data that it is protecting. 

* The data can only be accessed through the `RAII` guards returned from `lock` and `try_lock`.

    * Guarantees that the data is only ever accessed when the mutex is locked.

* `Mutex` implements `poisoning`.

    * A Mutex is considered `poisoned` whenever a thread panics while holding the mutex. 
    
    * Once a Mutex is `poisoned`, all other threads are unable to access the data by default.

---

## References

* [std::sync::Mutex - Rust Docs](https://doc.rust-lang.org/nightly/std/sync/struct.Mutex.html)

* [RAII - Resource Acquisition is Initialisation](https://en.wikipedia.org/wiki/Resource_acquisition_is_initialization)