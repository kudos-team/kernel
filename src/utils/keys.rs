use crate::print;
use crate::keyboard::KeyboardStream;
use futures_util::StreamExt;

#[allow(dead_code)]
pub async fn print_keypresses() {
    let mut kstream = KeyboardStream::new();
    while let Some(ev) = kstream.next().await {
        if let Some(c) = ev.unicode {
            print!("{}", c);
        } else if let Some(k) = ev.raw {
            print!("{:?}", k);
        }
    }
}

#[allow(dead_code)]
pub async fn choice(set: &[char]) -> char {
    let mut kstream = KeyboardStream::new();
    while let Some(ev) = kstream.next().await {
        if let Some(c) = ev.unicode {
            if set.contains(&c) {
                return c;
            }
        }
    }
    return '\0';
}
