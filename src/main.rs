#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

#![reexport_test_harness_main = "test_main"]

mod vga_buffer;

mod boot;
use crate::boot::on_boot;

#[cfg(test)]
mod tests;

use core::panic::PanicInfo;

/// This function is called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}


/// This function is called when testing
#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}


/// This function is called on init
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // Testing stuff
    println!("Loaded!");

    #[cfg(test)]
    test_main();

    #[cfg(not(test))]
    on_boot();

    loop {}
}
