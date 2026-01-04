use crate::{printLog, LogType};
use crate::utils::keys::print_keypresses;
use kudos::task::{Task, executor::Executor};

/// This function will run when running the main program
pub fn on_boot() {
    printLog!(LogType::Info, "Test info!");
    printLog!(LogType::Good, "Test good!");
    printLog!(LogType::Warn, "Test warn!");
    printLog!(LogType::Error, "Test bad!");

    let mut executor = Executor::new();
    executor.spawn(Task::new(print_keypresses()));
    executor.run();
}
