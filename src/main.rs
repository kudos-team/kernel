#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kudos::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(not(test))]
mod fancy;
#[cfg(not(test))]
mod boot;


use kudos::println;
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

/// This function is called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    kudos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kudos::test_panic_handler(info)
}


// This function is called on init
entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    kudos::init(boot_info);
    println!("Loaded!");

    #[cfg(test)]
    test_main();
    #[cfg(not(test))]
    boot::on_boot();

    kudos::hlt_loop();
}
