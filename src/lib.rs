#![no_std]
#![cfg_attr(test, no_main)]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;
pub mod allocator;
pub mod memory;

pub mod sigslt;

pub mod interrupts;
pub mod gdt;

pub mod serial;
pub mod vga_buffer;

pub mod task;

pub mod utils;
pub use utils::fancy::LogType;


pub trait Testable {
    fn run(&self) -> ();
}
impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}


pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}


/// Halts the CPU until the next instruction (more efficient than an infinite loop)
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}


use bootloader::BootInfo;
static mut INITED: bool = false;
/// Initialises everything necessary
pub fn init(boot_info: &'static BootInfo, fancy: bool) {
    unsafe {
        if INITED {
            return
        }
        INITED = true;
    }
    if fancy {
        printlg!("Loading...");
    }
    use x86_64::VirtAddr;

    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    if fancy {
        utils::fancy::clear_line();
        printlgln!(LogType::Good, "Loaded!");
    }
}


#[cfg(test)]
use bootloader::entry_point;
#[cfg(test)]
entry_point!(test_kernel_main);
// Entry point for `cargo test`
#[cfg(test)]
fn test_kernel_main(boot_info: &'static BootInfo) -> ! {
    init(boot_info);
    test_main();
    hlt_loop();
}


use core::panic::PanicInfo;
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

pub fn real_panic_handler(info: &PanicInfo) -> ! {
    println!();
    printlgln!(LogType::Error, "{}", info);
    hlt_loop();
}
