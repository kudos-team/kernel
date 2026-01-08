use alloc::{boxed::Box, vec::Vec};
use spin::Mutex;


// Define the Slot type as a boxed closure
type Slot<T> = Box<dyn Fn(&T) + Send + Sync + 'static>;

pub struct Signal<T = ()> {
    slots: Mutex<Vec<Slot<T>>>,
    default_slot: Option<Slot<T>>,
}

impl<T: 'static> Signal<T> {
    pub fn new() -> Self {
        Self { slots: Mutex::new(Vec::new()), default_slot: None }
    }
    pub fn new_default<F>(default: F) -> Self
        where
            F: Fn(&T) -> () + Sync + Send + 'static {
        Self {
            slots: Mutex::new(Vec::new()),
            // Store a boxed closure that returns a pinned boxed future
            default_slot: Some(Box::new(default)),
        }
    }

    // Connect a new closure (slot) to the signal
    pub fn connect<F>(&self, slot: F)
        where
            F: Fn(&T) -> () + Sync + Send + 'static {
        self.slots.lock().push(Box::new(slot));
    }

    // Emit the signal, calling all connected slots
    pub fn emit_with(&self, args: &T) {
        let slots = self.slots.lock();

        if slots.is_empty() {
            if let Some(default) = &self.default_slot {
                default(args);
            }
            return;
        }

        for slot in slots.iter() {
            slot(args);
        }
    }
}

// When no params, call emit with no arguments.
impl Signal<()> {
    pub fn emit(&self) {
        let slots = self.slots.lock();

        if slots.is_empty() {
            if let Some(default) = &self.default_slot {
                default(&());
            }
            return;
        }

        for slot in slots.iter() {
            slot(&());
        }
    }
}


// Macros for ease of use
#[macro_export]
macro_rules! GlobalSig{
    ($name:ident) => {
        lazy_static::lazy_static! {
            pub static ref $name: Signal = Signal::new();
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
            pub static ref $name: Signal = Signal::new_default($default);
        }
    };
    ($name:ident : $ty:ty, $default:expr) => {
        lazy_static::lazy_static! {
            pub static ref $name: Signal<$ty> = Signal::new_default($default);
        }
    };
}
