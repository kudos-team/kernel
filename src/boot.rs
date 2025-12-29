use kudos::{print, println};

/// This function will run when running the main program
pub fn on_boot() {
    print!("Hello, ");
    println!("world!");
    kudos::interrupts::breakpoint();
    println!("Still runs!");
    //panic!("Purposeful panic for testing");
}
