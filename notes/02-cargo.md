# Cargo - Basic Rust Projects


## [Cargo Projects](https://doc.rust-lang.org/cargo/index.html)

- [Create](https://doc.rust-lang.org/cargo/guide/creating-a-new-project.html)

    ```
    cargo new ${project_name} --bin
    ```

- **Build**

    ```
    cargo build
    ```

- **Cargo Nightly Build**

    ```
    rustup run nightly cargo build
    ```

- **Optimised Release Build_

    ```
    cargo build --release
    ```

- **Run**

    ```
    cargo run
    ```

- **Clean**

    ```
    cargo clean
    ```

- **Help**

    ```
    cargo --help
    ```

---

## [Cargo Dependency Management](https://doc.rust-lang.org/cargo/guide/dependencies.html)

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

