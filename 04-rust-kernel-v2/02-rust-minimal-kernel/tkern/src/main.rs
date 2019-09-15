// Do not use or link to standard library. 
// 
// https://doc.rust-lang.org/1.30.0/book/first-edition/using-rust-without-the-standard-library.html
//
#![no_std]

// Do no required a 'main' function.
//
#![no_main]

use core::panic::PanicInfo;

static MSG: &[u8] = b"Hello tkern!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in MSG.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

/// This function is called on panic.
//
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
