# tkern`

## Introduction

A simple operating system kernel written in `Rust`.

---

## Build

1. **Switch to rust nightly** : `rustup override add nightly`

2. **Check Version** - `rustc --version`

3. **Add rust source** : `rustup component add rust-src`

4. **Install `cargo-xbuild` for cross-compilation** : `cargo install cargo-xbuild`

5. **Build** : `cargo xbuild` or `cargo xbuild --target x86_64-tkern.json`

6. **Install `llvm-tools-preview`** : `rustup component add llvm-tools-preview`

7. **Install `bootimage`** : `cargo install bootimage --version "^0.7.3"`

8. **Create bootimage** : `cargo bootimage`

9. **Run in `QEMU`** : `qemu-system-x86_64 -drive format=raw,file=target/x86_64-tkern/debug/bootimage-tkern.bin`

    * or : `cargo xrun` (with `.cargo/config` run method).
