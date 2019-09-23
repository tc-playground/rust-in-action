# Global Statics

## Introduction

1. In Rust, `global variables` are called `static variables`.

    1. They can be `mutable`.

    2. They can be `immutable`.

2. Accessing an `immutable static variable` is safe.

3. Accessing an `mutable static variable` is unsafe.

4. Statics can be problematic with Rust’s `ownership` rules. 

    1. If two threads are accessing the same `mutable global variable`, it can cause a `data race`.

    > Using `static mut` is highly discouraged.