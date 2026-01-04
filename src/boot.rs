use kudos::{print, println};

use kudos::task::{Task, simple_executor::SimpleExecutor};

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}


/// This function will run when running the main program
pub fn on_boot() {
    let mut executor = SimpleExecutor::new();
    executor.spawn(Task::new(example_task()));
    executor.run();

    print!("Hello, ");
    println!("world!");
    /*kudos::interrupts::breakpoint();
    println!("Still runs!");*/
    //panic!("Purposeful panic for testing");
    /*loop {
        print!("-");
    }*/
}
