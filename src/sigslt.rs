use crate::task::{Task, executor::Executor};

use alloc::{boxed::Box, vec::Vec};
use spin::Mutex;
use core::future::Future;
use core::pin::Pin;


// Define the Slot type as a boxed closure
type Slot<T> = Box<dyn Fn(T) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> + Send + Sync + 'static>;

pub struct Signal<T = ()> {
    slots: Mutex<Vec<Slot<T>>>,
    default_slot: Option<Slot<T>>,
}

impl<T: 'static + Clone> Signal<T> {
    pub fn new() -> Self {
        Self { slots: Mutex::new(Vec::new()), default_slot: None }
    }
    pub fn new_default<F, Fut>(default: F) -> Self
        where
            F: Fn(T) -> Fut + Send + Sync + 'static,
            Fut: Future<Output = ()> + Send + 'static {
        Self {
            slots: Mutex::new(Vec::new()),
            // Store a boxed closure that returns a pinned boxed future
            default_slot: Some(Box::new(move |t| Box::pin(default(t)))),
        }
    }

    // Connect a new closure (slot) to the signal
    pub fn _connect<F, Fut>(&self, slot: F)
        where
            F: Fn(T) -> Fut + Send + Sync + 'static,
            Fut: Future<Output = ()> + Send + 'static {
        self.slots.lock().push(Box::new(move |t| { Box::pin(slot(t)) }));
    }

    // Emit the signal, calling all connected slots
    pub fn emit_with(&self, args: T)
            where T: Clone + Send + 'static {
        let slots = self.slots.lock();
        let mut executor = Executor::new();

        if slots.is_empty() {
            if let Some(default) = &self.default_slot {
                executor.spawn(Task::new(default(args)));
                executor.run();
            }
            return;
        }

        for slot in slots.iter() {
            executor.spawn(Task::new(slot(args.clone())));
        }
        executor.run();
    }
}

// When no params, call emit with no arguments.
impl Signal<()> {
    pub fn emit(&self) {
        let slots = self.slots.lock();
        let mut executor = Executor::new();

        if slots.is_empty() {
            if let Some(default) = &self.default_slot {
                executor.spawn(Task::new(default(())));
                executor.run();
            }
            return;
        }

        for slot in slots.iter() {
            executor.spawn(Task::new(slot(())));
        }
        executor.run();
    }
}


// Macros for ease of use
#[macro_export]
macro_rules! connect {
    ($signal:expr, $handler:expr) => {
        $signal._connect(move |t| {
            alloc::boxed::Box::pin(async move { $handler(&t).await })
        })
    };
}

#[macro_export]
macro_rules! GlobalSignal {
    ($name:ident) => {
        lazy_static::lazy_static! {
            pub static ref $name: Signal<()> = Signal::new();
        }
    };
    ($name:ident : $ty:ty) => {
        lazy_static::lazy_static! {
            pub static ref $name: Signal<$ty> = Signal<$ty>::new();
        }
    };
}
