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
* __Cargo Nightly Build__
  ```
  rustup run nightly cargo build
  ```
* __Optimised Release Build__
  ```
  cargo build --release
  ```
* __Run__
  ```
  cargo run
  ```
* __Clean__
  ```
  cargo clean
  ```
* __Help__
  ```
  cargo --help
  ```
---

## [Cargo Dependency Management(https://doc.rust-lang.org/cargo/guide/dependencies.html)

  ```
  [package]
  name = "date_server"
  version = "0.1.0"
  authors = ["Tim Langford <tim.langford@gmail.com>"]

  [dependencies]
  chrono = "0.4.0"
  rocket = "0.3.0"
  rocket_codegen = "0.3.0"
  serde = "1.0"
  serde_derive = "1.0"

  [dependencies.rocket_contrib]
  version = "0.3.0"
  default-features = false
  features = ["json"]
  ```

