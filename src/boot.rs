use kudos::{print, println};

/// This function will run when running the main program
pub fn on_boot(boot_info: &'static BootInfo) {
    use kudos::memory;
    use kudos::memory::BootInfoFrameAllocator;
    use x86_64::{structures::paging::Page, VirtAddr};


    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

    print!("Hello, ");
    println!("world!");
    /*kudos::interrupts::breakpoint();
    println!("Still runs!");*/
    //panic!("Purposeful panic for testing");
    /*loop {
        print!("-");
    }*/
}
