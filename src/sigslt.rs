use crate::task::{Task, executor::Executor};

use alloc::{boxed::Box, vec::Vec, sync::Arc};
use spin::Mutex;
use core::future::Future;
use core::pin::Pin;


// Define the Slot type as a boxed closure
type Slot<T> = Box<dyn Fn(Arc<T>) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>
    + Send + Sync + 'static>;
type Default<T> = Box<dyn Fn(Arc<T>) -> () + Send + Sync + 'static>;

pub struct Signal<T = ()> {
    slots: Mutex<Vec<Slot<T>>>,
    default_slot: Option<Default<T>>,
}

impl<T: 'static + Send + Sync> Signal<T> {
    pub fn new() -> Self {
        Self { slots: Mutex::new(Vec::new()), default_slot: None }
    }
    pub fn new_default(default: Default<T>) -> Self {
        Self {
            slots: Mutex::new(Vec::new()),
            default_slot: Some(default),
        }
    }

    // Connect a new closure (slot) to the signal
    pub fn _connect<F, Fut>(&self, slot: F)
        where
            F: Fn(Arc<T>) -> Fut + Send + Sync + 'static,
            Fut: Future<Output = ()> + Send + 'static {
        self.slots.lock().push(Box::new(move |t| { Box::pin(slot(t)) }));
    }


    // Emit the signal, calling all connected slots
    pub fn emit_with(&self, args: T)
            where T: Send + Sync + 'static {
        let slots = self.slots.lock();
        let arc = Arc::new(args);

        if slots.is_empty() {
            if let Some(default) = &self.default_slot {
                default(arc);
            }
            return;
        }

        let mut executor = Executor::new();
        for slot in slots.iter() {
            executor.spawn(Task::new(slot(arc.clone())));
        }
        executor.run();
    }
}

// When no params, call emit with no arguments.
impl Signal<()> {
    pub fn emit(&self) {
        let slots = self.slots.lock();

        if slots.is_empty() {
            if let Some(default) = &self.default_slot {
                default(().into());
            }
            return;
        }

        let mut executor = Executor::new();
        for slot in slots.iter() {
            executor.spawn(Task::new(slot(().into())));
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
    }
}

#[macro_export]
macro_rules! GlobalSig{
    ($name:ident) => {
        lazy_static::lazy_static! {
            pub static ref $name: Signal<()> = Signal::new();
        }
    };
    ($name:ident : $ty:ty) => {
        lazy_static::lazy_static! {
            pub static ref $name: Signal<$ty> = Signal::new();
        }
    };
}

#[macro_export]
macro_rules! GlobalSigDef {
    ($name:ident, $default:expr) => {
        lazy_static::lazy_static! {
            pub static ref $name: Signal<()> = Signal::new_default(alloc::boxed::Box::new($default));
        }
    };
    ($name:ident : $ty:ty, $default:expr) => {
        lazy_static::lazy_static! {
            pub static ref $name: Signal<$ty> = Signal::new_default(alloc::boxed::Box::new($default));
        }
    };
}
