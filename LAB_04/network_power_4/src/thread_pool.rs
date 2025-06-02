use std::future::Future;
use std::sync::{Arc, Mutex, Condvar};
use std::collections::VecDeque;
use std::thread;
use std::task::{Context, Poll, Waker};
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct ThreadPool {
    #[allow(dead_code)] // workers not used yet, but to features like join we will use it
    workers: Vec<thread::JoinHandle<()>>,
    queue: Arc<JobQueue>,
}

struct JobQueue {
    queue: Mutex<VecDeque<Job>>,
    condvar: Condvar,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(n: usize) -> Self {
        let queue = Arc::new(JobQueue {
            queue: Mutex::new(VecDeque::new()),
            condvar: Condvar::new(),
        });

        let mut workers = Vec::with_capacity(n);
        for _ in 0..n {
            let queue_clone = Arc::clone(&queue);
            workers.push(thread::spawn(move || loop {
                let job = {
                    let mut q = queue_clone.queue.lock().unwrap();
                    while q.is_empty() {
                        q = queue_clone.condvar.wait(q).unwrap();
                    }
                    q.pop_front()
                };
                if let Some(job) = job {
                    job();
                }
            }));
        }

        Self { workers, queue }
    }

    pub fn execute<F, X>(&self, f: F) -> ThreadPoolFuture<X>
    where
        F: FnOnce() -> X + Send + 'static,
        X: Send + 'static,
    {
        let result = Arc::new(Mutex::new(None));
        let waker_slot = Arc::new((Mutex::new(None::<Waker>), Condvar::new()));
        let ready = Arc::new(AtomicBool::new(false));

        let result_clone = result.clone();
        let waker_clone = waker_slot.clone();
        let ready_clone = ready.clone();

        let job = Box::new(move || {
            let output = f();
            *result_clone.lock().unwrap() = Some(output);
            ready_clone.store(true, Ordering::SeqCst);
            if let Some(w) = waker_clone.0.lock().unwrap().take() {
                w.wake();
            }
        });

        self.queue.queue.lock().unwrap().push_back(job);
        self.queue.condvar.notify_one();

        ThreadPoolFuture {
            result,
            ready,
            waker_slot,
        }
    }
}

pub struct ThreadPoolFuture<T> {
    result: Arc<Mutex<Option<T>>>,
    ready: Arc<AtomicBool>,
    waker_slot: Arc<(Mutex<Option<Waker>>, Condvar)>,
}

impl<T> Future for ThreadPoolFuture<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.ready.load(Ordering::SeqCst) {
            let mut result = self.result.lock().unwrap();
            Poll::Ready(result.take().unwrap())
        } else {
            let mut waker_lock = self.waker_slot.0.lock().unwrap();
            *waker_lock = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}
