#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

#![reexport_test_harness_main = "test_main"]

mod vga_buffer;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod serial;

#[cfg(not(test))]
mod boot;


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
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    tests::exit_qemu(tests::QemuExitCode::Failed);
    loop {}
}


/// This function is called when testing
#[cfg(test)]
pub fn test_runner(tests: &[&dyn tests::Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    tests::exit_qemu(tests::QemuExitCode::Success);
}


/// This function is called on init
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // Testing stuff
    println!("Loaded!");

    #[cfg(test)]
    test_main();

    #[cfg(not(test))]
    boot::on_boot();

    loop {}
}
