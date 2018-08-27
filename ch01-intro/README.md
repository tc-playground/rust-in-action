# Basic Rust Projects with Cargo

## Install Rust and Cargo
* [Install](https://www.rust-lang.org/en-US/install.html)
  ```
  curl https://sh.rustup.rs -sSf | sh
  # rustup install nightly
  # rustup run nightly cargo run
  ```

## [Cargo Projects](https://doc.rust-lang.org/cargo/index.html)
* [Create](https://doc.rust-lang.org/cargo/guide/creating-a-new-project.html)
  ```
  cargo new ${project_name} --bin
  ```
* __Build__
  ```
  cargo build
  ```
* __Run__
  ```
  cargo run
  ```
* __Optimised Release Build__
  ```
  cargo build --release
  ```
* __Clean__
  ```
  cargo clean
  ```
* __Help__
  ```
  cargo --help
  ```

