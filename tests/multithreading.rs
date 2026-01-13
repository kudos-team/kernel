#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kudos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use kudos::println;
use kudos::task::{Task, executor::Executor};

async fn async_number() -> u32 {
    42
}
async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}

#[test_case]
fn test_async1() {
    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(example_task()));
    executor.run();
}

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    kudos::init(boot_info, false);
    test_main();

    kudos::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kudos::test_panic_handler(info)
}
