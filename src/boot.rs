use kudos::utils::fancy;
use kudos::utils::keys::choice;
use kudos::{print, println, printlgln, LogType};
use kudos::task::{Task, executor::Executor};
extern crate alloc;

async fn main() {
    loop {
        let chararr = ['y', 'n'];
        let c = choice(&chararr).await;
        fancy::clear_line();
        if c == 'y' {
            print!("Yes!");
        } else {
            print!("No.");
        }
    }
}

/// This function will run when running the main program
pub fn on_boot() {
    use kudos::{connect, interrupts::{TimerIntSig, BreakpointIntSig}};
    connect!(TimerIntSig, async |_| {
        use kudos::print;
        print!(".");
    });
    connect!(BreakpointIntSig, async |_| {
        use kudos::printlgln;
        printlgln!(LogType::Error, "Breakpoint occurred!");
    });

    use kudos::interrupts::breakpoint;
    breakpoint();

    printlgln!(LogType::Info, "Test info!");
    printlgln!(LogType::Good, "Test good!");
    printlgln!(LogType::Warn, "Test warn!");
    printlgln!(LogType::Error, "Test bad!");
    println!("Press y/n");

    let mut executor = Executor::new();
    executor.spawn(Task::new(main()));
    executor.run();
}
