use kudos::print;
use kudos::task::keyboard::ScancodeStream;

use futures_util::stream::StreamExt;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};

#[allow(dead_code)]
pub async fn print_keypresses(scancodes: &mut ScancodeStream) {
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

#[allow(dead_code)]
pub async fn choice(scancodes: &mut ScancodeStream, set: &[char]) -> char {
    let mut keyboard = Keyboard::new(ScancodeSet1::new(),
        layouts::Us104Key, HandleControl::Ignore);

    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(DecodedKey::Unicode(c)) = keyboard.process_keyevent(key_event) {
                if set.contains(&c) {
                    return c;
                }
            }
        }
    }
    return '\0';
}
