#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kudos::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(not(test))]
mod boot;

#[cfg(not(test))]
#[allow(dead_code)]
fn test_main(){}

use kudos::LogType;
use kudos::printlgln;
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

/// This function is called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if cfg!(test) {
        kudos::test_panic_handler(info)
    } else {
        kudos::real_panic_handler(info)
    }
}


// This function is called on init
entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {

    kudos::init(boot_info, true);
    printlgln!(LogType::Good, "Loaded!");

    // --- ATA DRIVER SMOKE TEST ---

    printlgln!(LogType::Info, "Probing for ATA devices...");



        // Use the full path to your new module and function
        match kudos::drivers::ata::identify() {
            Ok(identity) => {
                let model = match core::str::from_utf8(&identity.model_number) {
                    Ok(s) => s.trim(), // .trim() removes any padding spaces or nulls
                    Err(_) => "Invalid UTF-8 in model",
                };

                // Now do the same for the serial number
                let serial = match core::str::from_utf8(&identity.serial_number) {
                    Ok(s) => s.trim(),
                    Err(_) => "Invalid UTF-8 in serial",
                };

                printlgln!(LogType::Good, "ATA Drive Found!");
                printlgln!(LogType::Info, "  Model:  {}", model);
                printlgln!(LogType::Info, "  Serial: {}", serial);
            }
            Err(err) => {
                // Failure! Print the error message you wrote.
                printlgln!(LogType::Error, "ATA probe failed: {}", err);
            }
        }


    x86_64::instructions::interrupts::enable();
    printlgln!(LogType::Info, "Interrupts Enabled.");

    #[cfg(test)]
    test_main();
    #[cfg(not(test))]
    boot::on_boot();


    kudos::hlt_loop();
}