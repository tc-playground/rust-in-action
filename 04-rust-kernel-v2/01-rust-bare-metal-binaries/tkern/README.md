# Temkern`

## Introduction

A simple operating system kernel written in `Rust`.

---

## Build

### Via `Triple Targets`

1. Install required `triple target` - `rustup target add thumbv7em-none-eabihf`

2. Run `cargo build --target thumbv7em-none-eabihf`


### Via direct `Linker Arguments`

- **Linux** - `cargo rustc -- -C link-arg=-nostartfiles`

- **Windows** - `cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"`

- **macOS** - `cargo rustc -- -C link-args="-e __start -static -nostartfiles"`


### Via file configured `Linker Arguments`

1. Create `.cargo/config`

    ```
    [target.'cfg(target_os = "linux")']
    rustflags = ["-C", "link-arg=-nostartfiles"]

    [target.'cfg(target_os = "windows")']
    rustflags = ["-C", "link-args=/ENTRY:_start /SUBSYSTEM:console"]

    [target.'cfg(target_os = "macos")']
    rustflags = ["-C", "link-args=-e __start -static -nostartfiles"]
    ```

2. Run `cargo build`
