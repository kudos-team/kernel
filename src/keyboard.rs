use core::{pin::Pin, task::{Context, Poll}};
use core::sync::atomic::Ordering;

use crate::task::keyboard::{get_ScancodeStream, SUPER_DOWN};
use pc_keyboard::{layouts, KeyCode, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use futures_util::{StreamExt, stream::Stream};


pub struct KeyEvent {
    pub unicode: Option<char>,
    pub raw: Option<KeyCode>,

    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub caps: bool,
    pub souper: bool,
}

pub struct KeyboardStream {
    keyboard: Keyboard<layouts::Us104Key, ScancodeSet1>,
}

impl KeyboardStream {
    pub fn new() -> Self {
        Self {
            keyboard: Keyboard::new(
                ScancodeSet1::new(),
                layouts::Us104Key,
                HandleControl::Ignore,
            ),
        }
    }

    pub fn clear(&self) {
        get_ScancodeStream().lock().clear_queue();
    }
}

impl Stream for KeyboardStream {
    type Item = KeyEvent;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context)
        -> Poll<Option<Self::Item>>
    {
        loop {
            let scancode = match Pin::new(&mut get_ScancodeStream().lock()).poll_next_unpin(cx) {
                Poll::Ready(Some(s)) => s,
                Poll::Ready(None) => return Poll::Ready(None),
                Poll::Pending => return Poll::Pending,
            };

            if let Ok(Some(event)) = self.keyboard.add_byte(scancode) {
                if let Some(key) = self.keyboard.process_keyevent(event) {
                    let (unicode, raw) = match key {
                        DecodedKey::Unicode(c) => (Some(c), None),
                        DecodedKey::RawKey(k) => (None, Some(k)),
                    };
                    let m = self.keyboard.get_modifiers();
                    return Poll::Ready(Some(KeyEvent {
                        unicode,
                        raw,

                        shift: m.is_shifted(),
                        ctrl:  m.is_ctrl(),
                        alt:   m.is_alt() || m.is_altgr(),
                        caps:  m.is_caps(),
                        souper:SUPER_DOWN.load(Ordering::Relaxed),
                    }));
                }
            }
        }
    }
}
