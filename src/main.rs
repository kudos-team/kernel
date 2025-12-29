#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kudos::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(not(test))]
mod boot;


use kudos::println;
use core::panic::PanicInfo;

/// This function is called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kudos::test_panic_handler(info)
}


/// This function is called on init
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Loaded!");

    #[cfg(test)]
    test_main();
    #[cfg(not(test))]
    boot::on_boot();

    loop {}
}
