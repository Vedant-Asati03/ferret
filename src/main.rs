#![no_std] // attribute to not use rust standard library
#![no_main] // we will need to implement our own entry point, so we disable all rust-level entry points

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

#[unsafe(no_mangle)] // we dont want the name of this function to be mangled, so we add this trait
pub extern "C" fn _start() -> ! {
    // we named the function _start because the linker by-default looks for a function named
    // _start, this function is the custom entry point we defined
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


