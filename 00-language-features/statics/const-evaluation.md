# Constant Evaluation

## Introduction

1. `Constant evaluation` is the process of computing values at `compile time`.

    1. Reduces the workload or binary size at runtime by precomputing complex operations at compile time and only storing the result.

2. The component of the `Rust compiler` that evaluates such initialization expressions is called the `const evaluator`.

    1. [Const Evaluation](https://rust-lang.github.io/rustc-guide/const-eval.html) 

---

## References

* [Const Evaluation](https://rust-lang.github.io/rustc-guide/const-eval.html) 

* [`lazy_static`](https://docs.rs/lazy_static/1.0.1/lazy_static/)
