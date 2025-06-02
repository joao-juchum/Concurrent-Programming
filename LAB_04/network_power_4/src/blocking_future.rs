use std::{
    cell::Cell,
    thread::{self, JoinHandle},
    future::Future,
    task::{Poll, Context},
    pin::Pin,
};

pub struct BlockingFuture<R, F: FnOnce() -> R> {
    funct: Cell<Option<F>>,
    thread: Cell<Option<*mut JoinHandle<R>>>,
}

impl<R, F: FnOnce() -> R> BlockingFuture<R, F> {
    pub fn new(f: F) -> Self {
        Self {
            funct: Cell::new(Some(f)),
            thread: Cell::new(None),
        }
    }
}

impl<R: Send + 'static, F: FnOnce() -> R + Send + 'static> Future for BlockingFuture<R, F> {
    type Output = R;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Init a thread if necessary
        if self.thread.get().is_none() {
            if let Some(f) = self.funct.take() {
                let handle = Box::new(thread::spawn(f));
                self.thread.set(Some(Box::into_raw(handle)));
            }
        }

        // Verify if the thread was finished
        if let Some(ptr) = self.thread.get() {
            let boxed_handle = unsafe { Box::from_raw(ptr) };
            match boxed_handle.join() {
                Ok(result) => Poll::Ready(result),
                Err(_) => panic!("Thread panicked"),
            }
        } else {
            Poll::Pending
        }
    }
}
