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
