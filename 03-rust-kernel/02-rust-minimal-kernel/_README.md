# [Minimal Rust Kernel](https://os.phil-opp.com/minimal-rust-kernel/)

## Introduction

1. Write a minimal boot up image in `Rust`.

---

## General Boot Process

1. Turn on a computer.

2. Load and begin executing firmware code that is stored in `BIOS` or `UEFI` ROM. 

3. This code performs a power-on self-test.

4. Detects available RAM.

5. Pre-initializes the CPU and hardware. 

6. Look for a bootable disk.

7. Starts booting the operating system kernel.

---

## BIOS Boot

1. Turn on a computer. 

2. Load the `BIOS` from some special flash memory located on the motherboard. 

3. The `BIOS` runs self test and initialization routines of the hardware, 

4. The `BIOS` looks for bootable disks. 

5. The `BIOS` loads and transfers the control to the bootloader a 512-byte portion of executable code stored at the start of the disk. 

    1. `bootloaders` are usually larger than 512 bytes.
    
    2. `bootloaders` are split into a two stage, the first fits into 512 bytes and loads and executed the second.

6. The `bootloader` determines thew location of the `kernel image` on disk and loads it. 

7. The `bootloader` needs to:

    1. Switch the `CPU` from `16-bit real mode` to `32-bit protected mode`. 
    
    2. Switch the `CPU` from `32-bit protected mode` to `64-bit long mode` (64-bit registers and complete main memory are now available).
    
    3. Query information (such as a `memory map`) from the `BIOS` and pass it to the `OS kernel`.

> NB: This example uses the [`bootloader`](https://github.com/rust-osdev/bootimage) project to handle this process.


### `Multiboot` BootLoader Standard

1. [`Multiboot`](https://wiki.osdev.org/Multiboot) - a GNU standard for bootloaders.

    - The standard defines an interface between the `bootloader` and `operating system`.
    
    - Any Multiboot compliant bootloader can load any Multiboot compliant operating system.

    - Only  supports `32-bit protected mode`.

    - kernel needs to be linked with an [adjusted default page size](https://wiki.osdev.org/Multiboot#Multiboot_2).

    - [Boot information specification](https://www.gnu.org/software/grub/manual/multiboot/multiboot.html#Boot-information-format)

2. [`GRUB2](https://en.wikipedia.org/wiki/GNU_GRUB) is an implementation of multiboot.

---

## Compiling Stand Alone (Bare Metal) Binary

* `Cargo` builds for the host system by default.

* `Cargo` supports different target systems through the `--target` parameter.

    * The target is described by a `target triple`, which describes:
    
        1. The CPU architecture.
        
        2. The vendor. 
        
        3. The operating system, and the ABI.
    
    * Examples: `x86_64-unknown-linux-gnu`, `arm-linux-androideabi`.

* `Cargo` use `LLVM` under the hood. The configuration options can be specified by a configuration file.

    ```
    {
        "llvm-target": "x86_64-unknown-none",                       # LLVM target triple.
        "data-layout": "e-m:e-i64:64-f80:128-n8:16:32:64-S128",
        "arch": "x86_64",
        "target-endian": "little",
        "target-pointer-width": "64",
        "target-c-int-width": "32",
        "os": "none",                                               # No OS.
        "executables": true,
        "linker-flavor": "ld.lld",                                  # Use the LLD linker.
        "linker": "rust-lld",                                       # Use the LLD linker.
        "panic-strategy": "abort",                                  # The target doesn't support stack unwinding on panic, 
        "disable-redzone": true,                                    # Safely handle interrupts. 
                                                                    # Disable a certain stack pointer optimization called the “red zone”.
        "features": "-mmx,-sse,+soft-float"                         # Disable `mmx` and `sse`. Inefficient for kernel programs due to large registers.
                                                                    # Enable `soft floats` as `mmx`/`sse` has been disabled.
    }
    ```

    * The fields relate to various build options:

        1. `LVM code generation configuration fields`

        2. `Rust conditional compilation fields`.

        3. `Crate build fields` - e.g. `linker` options.

* `Cargo xbuild` is a wrapper around `Cargo build` that recompiles core libraries e.g. `core library` for the defined `triple target`.

* `Cargo xbuild` __cross-compiles__ the `core`, `compiler_builtin`, and `alloc` libraries for our new custom target.

---

## Implementing VGA Text Buffer

* [VGA Text Buffer](https://en.wikipedia.org/wiki/VGA-compatible_text_mode)

* `VGA Text Buffer` is:

    1. A special memory area mapped to the VGA hardware that contains the contents displayed on screen. 
    
    2. Normally consists of 25 lines that each contain 80 character cells. 
    
    3. Each character cell displays an ASCII character with some foreground and background colors.

    4. Located at memory address `0xb8000`.

---

## Booting and Running the Kernel

* Make a "kernel" bootable by linking it with some `bootloader` code.

* The [`bootloader`](https://github.com/rust-osdev/bootloader) crate implements a simple (non multiboot) boot loader.

    * Implements a basic BIOS bootloader without any C dependencies, just Rust and inline assembly

* The [`bootimage`](https://github.com/rust-osdev/bootimage) links the `bootloader` crate to an executable.

    1. Compiles kernel into an [ELF](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format) file.

    2. It compiles the bootloader dependency as a standalone executable.

    3. It links the bytes of the kernel ELF file to the bootloader.

* Run directly in [`QEMU`](https://www.qemu.org/)

    1. `qemu-system-x86_64 -drive format=raw,file=target/x86_64-tkern/debug/bootimage-tkern.bin`

* Run from a real USB

    1. `dd if=target/x86_64-tkern/debug/bootimage-tkern.bin of=/dev/usb-driver && sync`

---

## References

* [Minimal Rust Kernel](https://os.phil-opp.com/minimal-rust-kernel/)

* [Minimal Multiboot Kernel using GRUB](https://os.phil-opp.com/multiboot-kernel/)

* [`cargo-xbuild](https://github.com/rust-osdev/cargo-xbuild) - Cross compile Rust.

* [LLVM](https://llvm.org/)

    * [LLD Linker](https://lld.llvm.org/)

* [Disable Red Zone](https://os.phil-opp.com/red-zone/)
    * [C/C++ Stack Unwinding](https://www.bogotobogo.com/cplusplus/stackunwinding.php)

* [Disable SIMD](https://os.phil-opp.com/disable-simd/)

    * [Single Instruction Multiple Data](https://en.wikipedia.org/wiki/SIMD)

* [`bootloader`](https://github.com/rust-osdev/bootloader) - The `bootloader` crate is a non-multiboot bootloader.

* [`bootimage`](https://github.com/rust-osdev/bootimage) - Link `bootimage` crate to he required executable.


