//! toki-no: A minimal and fast async runtime.
use std::future::Future;

/// The `toki-no` runtime, responsible for executing asynchronous tasks.
///
/// This struct will hold the state for the task scheduler and I/O reactor.
#[derive(Debug, Default)]
pub struct Runtime {
    // Internal runtime state will go here.
}

impl Runtime {
    /// Creates a new `toki-no` runtime instance.
    pub fn new() -> Self {
        Runtime {
            // Initialization logic for the executor and reactor will go here.
        }
    }

    /// Runs a future to completion on the runtime.
    ///
    /// This is the entry point for blocking on a future's result.
    ///
    /// # Panics
    ///
    /// This function will currently panic as the executor is not implemented.
    pub fn block_on<F: Future>(&self, _future: F) -> F::Output {
        // The actual executor loop will be implemented here.
        unimplemented!("The `block_on` executor is not yet implemented.");
    }
}

/// Spawns a new asynchronous task to be run on the runtime.
///
/// # Panics
///
/// This function will currently panic as task spawning is not implemented.
pub fn spawn<F>(_future: F)
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    // Logic to add the future to the runtime's task queue will go here.
    unimplemented!("The `spawn` function is not yet implemented.");
}
