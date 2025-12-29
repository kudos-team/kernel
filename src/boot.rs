use kudos::{print, println};

/// This function will run when running the main program
pub fn on_boot() {
    print!("Hello, ");
    println!("world!");
    println!("On a new line!");
    //panic!("Purposeful panic for testing");
    kudos::interrupts::breakpoint();
}
