use kudos::{print, println};

use kudos::task::{Task, executor::Executor, keyboard::ScancodeStream};

use futures_util::stream::StreamExt;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
pub async fn print_keypresses() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(ScancodeSet1::new(),
        layouts::Us104Key, HandleControl::Ignore);

    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => print!("{}", character),
                    DecodedKey::RawKey(key) => print!("{:?}", key),
                }
            }
        }
    }
}


/// This function will run when running the main program
pub fn on_boot() {
    print!("Hello, ");
    println!("world!");
    /*kudos::interrupts::breakpoint();
    println!("Still runs!");*/
    //panic!("Purposeful panic for testing");
    /*loop {
        print!("-");
    }*/

    let mut executor = Executor::new();
    executor.spawn(Task::new(print_keypresses()));
    executor.run();
}
