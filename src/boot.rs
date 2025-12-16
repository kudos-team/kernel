use crate::{println};

/// This function will run when running the main program
pub fn on_boot() {
    println!("Hello world!");
    panic!("Some panic message");
}
