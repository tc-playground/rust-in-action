# [VGA Text Mode](https://os.phil-opp.com/vga-text-mode/)

## Introduction

1. Add an interface to encapsulate the `VGA text buffer` with the `unsafe` methods in the buffer.

2. Add support for a Rust like `formatted printing macro`.

---

## VGA Module

1. Support [`code page 437`](https://en.wikipedia.org/wiki/Code_page_437).

2. The `VGA Buffer` is (`memory-mapped`)[https://en.wikipedia.org/wiki/Memory-mapped_I/O] to `0xb8000`

    1. `Memory-mapped I/O` uses the same address space to address both memory and I/O devices. 
    
    2. The memory and registers of the I/O devices are mapped to (associated with) address values. 
    
    3. "hen an address is accessed by the CPU, it may refer to a portion of physical RAM, or it can instead refer to memory of the I/O device.
    
    4. CPU instructions used to access the memory can also be used for accessing devices.

3. Encapsulate `unsafe` memory `screen buffer` operations behind an interface.

4. Uses `volatile` methods (via `volatile` crate) to prevent compiler optimisations breaking VGA memory-mapped IO  write operations.

5. Implement `core::fmt::Write` trait to provide `formatted print` operations.

6. Provides a single static global instance of the VGA buffer `Writer` interface:

    1. Provide and instance of `Writer` that can be used from other modules without carrying an instance around.

    1. 


---





---

## References


