use crate::{printlgln, LogType};
use crate::utils::fancy;
use crate::utils::keys::choice;
use kudos::{print, println};
use kudos::task::{Task, executor::Executor, keyboard::ScancodeStream};
extern crate alloc;

async fn main() {
    let mut scancodes = ScancodeStream::new();
    loop {
        let chararr = ['y', 'n'];
        let c = choice(&mut scancodes, &chararr).await;
        fancy::clear_line();
        if c == 'y' {
            print!("Yes!");
        } else {
            print!("No.");
        }
    }
}

async fn timer_int(_: &()) {
    use kudos::print;
    print!(".");
}

/// This function will run when running the main program
pub fn on_boot() {
    use kudos::{connect, interrupts::TimerIntSig};
    connect!(TimerIntSig, timer_int);
    printlgln!(LogType::Info, "Test info!");
    printlgln!(LogType::Good, "Test good!");
    printlgln!(LogType::Warn, "Test warn!");
    printlgln!(LogType::Error, "Test bad!");
    println!("Press y/n");

    let mut executor = Executor::new();
    executor.spawn(Task::new(main()));
    executor.run();
}
