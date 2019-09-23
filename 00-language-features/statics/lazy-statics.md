# Lazy Static Instance

## Introduction

1. `Constant evaluation` is the process of computing values at compile time.

    1. Reduces the workload or binary size at runtime by precomputing complex operations at compiletime and only storing the result.

2. The component of the `Rust compiler` that evaluates such initialization expressions is called the `const evaluator`.

    1. [Const Evaluation](https://rust-lang.github.io/rustc-guide/const-eval.html) 

3. In Rust `statics` are initialized at __compile time__, in contrast to `normal variables` that are initialized at __run time__.

    1. The `const evaluator` is not able to convert `raw pointers` to `references` at compile time.

4. The current solution to this is to use the `lazy_static` crate.

    1. A macro for declaring lazily evaluated statics.

    2. Makes it is possible to have statics that require code to be executed at runtime in order to be initialized. 
    
    3. This allows anything requiring __heap allocations__, like vectors or hash maps, to be computed.
    
    4. This allows anything requiring __function calls__ to be computed.

---

## Example

1. This will result in a compile error `calls in statics are limited to constant functions, tuple structs and tuple variants`: 

    ```rust
    use volatile::Volatile;

    struct Buffer {
        chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
    }

    pub static WRITER: Writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };
    ```

    > This fails because of the use of `unsafe raw pointers`.

2. Import `lazy_static` into `Cargo.toml` (this example does not link to the `std_lib`): 

    ```toml
    [dependencies.lazy_static]
    version = "1.0"
    features = ["spin_no_std"]
    ```

3. Initialise the public static Writer with the `lazy_static!` macro:

    ```rust
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref WRITER: Writer = Writer {
            column_position: 0,
            color_code: ColorCode::new(Color::Yellow, Color::Black),
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        };
    }
    ```

    1. This Writer is still pretty useless since it is `immutable` (requires a pointer `&mut self`).

    2. Making it a `mutable static` is considered bad practice and defeats the purpose of Rust.

    3. Providing `interior mutability` via a relevant `smart pointer` (`RefCell`, `UnsafeCell`) or a `SpinLock` is required.

---

## References

* [Const Evaluation](https://rust-lang.github.io/rustc-guide/const-eval.html) 

* [`lazy_static`](https://docs.rs/lazy_static/1.0.1/lazy_static/