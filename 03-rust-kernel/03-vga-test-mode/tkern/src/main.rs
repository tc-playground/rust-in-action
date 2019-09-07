// Do not use or link to standard library. 
// 
// https://doc.rust-lang.org/1.30.0/book/first-edition/using-rust-without-the-standard-library.html
//
#![no_std]

// Do no required a default 'main' function.
#![no_main]

// Import core PanicInfo.
use core::panic::PanicInfo;

// Import the VGA Buffer Module.
mod vga_buffer;

// Main -----------------------------------------------------------------------
//

// The main function.
//
#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();
    loop {}
}

/// This function is called on panic.
//
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

