use crate::{printLog, LogType};
use crate::utils::fancy;
use crate::utils::keys::choice;
use kudos::{print, println};
use kudos::task::{Task, executor::Executor, keyboard::ScancodeStream};

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

/// This function will run when running the main program
pub fn on_boot() {
    printLog!(LogType::Info, "Test info!");
    printLog!(LogType::Good, "Test good!");
    printLog!(LogType::Warn, "Test warn!");
    printLog!(LogType::Error, "Test bad!");
    println!("Press y/n");

    let mut executor = Executor::new();
    executor.spawn(Task::new(main()));
    executor.run();
}
