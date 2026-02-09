use crate::println;

use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;

use spin::Mutex;

use core::{
    pin::Pin,
    task::{Poll, Context},
    sync::atomic::{AtomicBool, Ordering},
};
use futures_util::task::AtomicWaker;
use futures_util::stream::Stream;


pub static SUPER_DOWN: AtomicBool = AtomicBool::new(false);

static WAKER: AtomicWaker = AtomicWaker::new();

static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();

/// Called by the keyboard interrupt handler
pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        match scancode {
            0x5B => SUPER_DOWN.store(true, Ordering::Relaxed),  // LSuper down
            0xDB => SUPER_DOWN.store(false, Ordering::Relaxed), // LSuper up
            0x5C => SUPER_DOWN.store(true, Ordering::Relaxed),  // RSuper down
            0xDC => SUPER_DOWN.store(false, Ordering::Relaxed), // RSuper up
            _ => {}
        }
        if let Err(_) = queue.push(scancode) {
            println!("WARNING: scancode queue full; dropping keyboard input");
        } else {
            WAKER.wake();
        }
    } else {
        println!("WARNING: scancode queue uninitialized");
    }
}

pub struct ScancodeStream {
    _private: (),
}
impl ScancodeStream {
    fn new() -> Self {
        SCANCODE_QUEUE.try_init_once(|| ArrayQueue::new(100))
            .expect("ScancodeStream::new should only be called once");
        ScancodeStream { _private: () }
    }

    pub fn init() {
        SCANCODE_STREAM
            .try_init_once(|| Mutex::new(ScancodeStream::new()))
            .expect("ScancodeStream::init called twice");
    }

    pub fn clear_queue(&self) {
        let queue = SCANCODE_QUEUE
            .try_get()
            .expect("scancode queue not initialized");

        while queue.pop().is_some() {}
    }
}

impl Stream for ScancodeStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
        let queue = SCANCODE_QUEUE
            .try_get()
            .expect("scancode queue not initialized");

        // fast path
        if let Some(scancode) = queue.pop() {
            return Poll::Ready(Some(scancode));
        }

        WAKER.register(&cx.waker());
        match queue.pop() {
            Some(scancode) => {
                WAKER.take();
                Poll::Ready(Some(scancode))
            }
            None => Poll::Pending,
        }
    }
}

static SCANCODE_STREAM: OnceCell<Mutex<ScancodeStream>> = OnceCell::uninit();
#[allow(non_snake_case)]
pub fn get_ScancodeStream() -> &'static Mutex<ScancodeStream> {
    SCANCODE_STREAM
        .try_get()
        .expect("ScancodeStream not initialised")
}
