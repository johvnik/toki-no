//! toki-no: A minimal and fast async runtime.

use pin_utils::pin_mut;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;
use std::time::{Duration, Instant};

// ## Public API ##

/// The `toki-no` async runtime.
///
/// The runtime is the entry point for running async code and holds
/// state for managing tasks and resources.
#[derive(Default)]
pub struct Runtime;

impl Runtime {
    /// Creates a new `Runtime` instance with default configuration.
    pub fn new() -> Self {
        Runtime {}
    }

    /// Runs a future to completion on the current thread.
    /// This function will block the calling thread until the given future is complete.
    pub fn block_on<F: Future>(&self, future: F) -> F::Output {
        // Pin the future on the stack. `pin_mut!` is a safe macro.
        pin_mut!(future);

        let thread = thread::current();
        let waker = waker_fn::waker_fn(move || thread.unpark());
        let mut context = Context::from_waker(&waker);

        loop {
            match future.as_mut().poll(&mut context) {
                Poll::Ready(output) => return output,
                Poll::Pending => thread::park(),
            }
        }
    }
}

/// A future that completes after a specified duration.
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Sleep {
    target_time: Instant,
}

/// Creates a new future that will complete after the given `duration`.
pub fn sleep(duration: Duration) -> Sleep {
    Sleep {
        target_time: Instant::now() + duration,
    }
}

impl Future for Sleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.target_time {
            return Poll::Ready(());
        }

        // Schedule the waker to be called when the timer fires.
        let waker = cx.waker().clone();
        let target_time = self.target_time;
        thread::spawn(move || {
            let now = Instant::now();
            if now < target_time {
                thread::sleep(target_time - now);
            }
            waker.wake();
        });

        Poll::Pending
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn it_works_with_a_simple_future() {
        // 1. Create the runtime
        let rt = Runtime::new();

        let future = async { 5 };

        // 2. Call the block_on method
        assert_eq!(rt.block_on(future), 5);
    }

    #[test]
    fn it_works_with_a_future_that_yields() {
        // 1. Create the runtime
        let rt = Runtime::new();

        let future = async {
            println!("Going to sleep...");
            sleep(Duration::from_millis(50)).await;
            println!("Woke up!");
            10
        };

        // 2. Call the block_on method
        assert_eq!(rt.block_on(future), 10);
    }
}
