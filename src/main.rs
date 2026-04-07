#![no_std] // attribute to not use rust standard library
#![no_main] // we will need to implement our own entry point, so we disable all rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(minimal_rust_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use minimal_rust_kernel::println;

#[unsafe(no_mangle)] // we dont want the name of this function to be mangled, so we add this trait
pub extern "C" fn _start() -> ! {
    // we named the function _start because the linker by-default looks for a function named
    // _start, this function is the custom entry point we defined
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    minimal_rust_kernel::test_panic_handler(info)
}
