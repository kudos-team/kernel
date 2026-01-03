use crate::BootInfo;
use kudos::{print, println};

extern crate alloc;
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};

/// This function will run when running the main program
pub fn on_boot(boot_info: &'static BootInfo) {
    use kudos::allocator;
    use kudos::memory::{self, BootInfoFrameAllocator};
    use x86_64::VirtAddr;

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");


    // allocate a number on the heap
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    // create a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));


    print!("Hello, ");
    println!("world!");
    /*kudos::interrupts::breakpoint();
    println!("Still runs!");*/
    //panic!("Purposeful panic for testing");
    /*loop {
        print!("-");
    }*/
}
