use crate::BootInfo;
use kudos::{print, println};

/// This function will run when running the main program
pub fn on_boot(boot_info: &'static BootInfo) {
    use kudos::memory;
    use kudos::memory::BootInfoFrameAllocator;
    use x86_64::VirtAddr;


    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut _mapper = unsafe { memory::init(phys_mem_offset) };
    let mut _frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    print!("Hello, ");
    println!("world!");
    /*kudos::interrupts::breakpoint();
    println!("Still runs!");*/
    //panic!("Purposeful panic for testing");
    /*loop {
        print!("-");
    }*/
}
