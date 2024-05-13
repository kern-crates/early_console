use core::cell::{RefCell, RefMut};

/// Safety:
/// EarlyDev only can be used in early-stage of boot.
/// At that time, there's only one running thread.
/// When entering multi-task, disable earlycon and switch to formal console.
pub struct EarlyDev<T> {
    inner: RefCell<T>,
}

impl<T> EarlyDev<T> {
    pub const fn new(inner: T) -> Self {
        Self {
            inner: RefCell::new(inner),
        }
    }

    pub fn get_mut(&self) -> RefMut<'_, T> {
        self.inner.borrow_mut()
    }
}

unsafe impl<T> Sync for EarlyDev<T> {}
