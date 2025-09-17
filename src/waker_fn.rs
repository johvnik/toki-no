//! Convert closures into wakers.

extern crate alloc;

use std::{
    sync::Arc,
    task::{Wake, Waker},
};

/// Converts a closure into a `Waker`.
///
/// The closure gets called every time the waker is woken.
/// A `Waker` is just a fancy callback. This function converts regular closures into wakers.
pub fn waker_fn<F: Fn() + Send + Sync + 'static>(f: F) -> Waker {
    Waker::from(Arc::new(WakerHelper(f)))
}

/// A private helper struct that holds the closure and implements the `Wake` trait.
struct WakerHelper<F>(F);

impl<F: Fn() + Send + Sync + 'static> Wake for WakerHelper<F> {
    fn wake(self: Arc<Self>) {
        // Call the stored closure.
        (self.0)();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        // Call the stored closure by reference.
        (self.0)();
    }
}
