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

## References

* [Minimal Rust Kernel](https://os.phil-opp.com/minimal-rust-kernel/)

* [Minimal Multiboot Kernel using GRUB](https://os.phil-opp.com/multiboot-kernel/)

* [`bootloader`](https://github.com/rust-osdev/bootimage)

