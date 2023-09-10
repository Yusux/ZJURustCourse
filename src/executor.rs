use std::{
    cell::RefCell,
    task::Wake,
    sync::{Arc, Mutex, mpsc},
    future::Future,
    task::{Context, Poll, Waker},
    thread::JoinHandle,
};

use async_lock::OnceCell;
use futures::future::BoxFuture;
use scoped_tls::scoped_thread_local;

use crate::waker::Signal;

pub(crate) static GLOBAL_EXECUTOR: Executor = Executor::new_const();
static mut THREADS_NUM: usize = 0;
scoped_thread_local!(pub(crate) static EX: Executor);

// package future and signal together as Task
pub struct Task {
    future: RefCell<BoxFuture<'static, ()>>,
    signal: Arc<Signal>,
}

// null implement Send and Sync
unsafe impl Send for Task {}

unsafe impl Sync for Task {}

// implement Wake for Task
impl Wake for Task {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        let _ = GLOBAL_EXECUTOR.threads_pool().execute(self.clone());
        self.signal.notify();
    }
}

// implement Worker to execute Task in a thread
struct Worker {
    _wid: usize,
    wthread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc::<Mutex<mpsc::Receiver<Option<Arc<Task>>>>>) -> Self {
        // spawn a thread to execute Task
        let thread = std::thread::spawn(move || {
            loop {
                let task = receiver.lock().unwrap().recv().unwrap();
                // println!("worker {} got a task {:?}", id, task.is_some());
                match task {
                    Some(task) => {
                        // println!("worker {} got a task", id);
                        let waker = Waker::from(task.clone());
                        let mut cx = Context::from_waker(&waker);
                        let _ = task.future.borrow_mut().as_mut().poll(&mut cx);
                    },
                    // None means the thread should stop
                    None => {
                        // println!("worker {} got a None", id);
                        break;
                    },
                }
            }
        });

        Worker { _wid: id, wthread: Some(thread) }
    }
}

// implement ThreadsPool to manage Workers
struct ThreadsPool {
    workers: Vec<Worker>,
    max_workers: usize,
    sender: mpsc::Sender<Option<Arc<Task>>>,
}

impl ThreadsPool {
    fn new(max_workers: usize) -> Self {
        if max_workers == 0 {
            panic!("max_workers must be greater than 0");
        }
        let (sender, receiver) = mpsc::channel::<Option<Arc<Task>>>();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(max_workers);
        for i in 0..max_workers {
            workers.push(Worker::new(i, receiver.clone()));
        }
        ThreadsPool { workers, max_workers, sender }
    }

    // execute Task
    // Poll::Pending means the Task is not finished, but ignored in this implementation
    fn execute(&self, task: Arc<Task>) -> Poll<()> {
        self.sender.send(Some(task)).unwrap();
        Poll::Pending
    }
}

// implement Drop for ThreadsPool
impl Drop for ThreadsPool {
    fn drop(&mut self) {
        // println!("drop ThreadsPool");
        // send None to each thread to stop it
        for _ in 0..self.max_workers {
            let _ = self.sender.send(None);
            // println!("send None");
        }
        // drop the threads
        for worker in &mut self.workers {
            if let Some(thread) = worker.wthread.take() {
                let _ = thread.join();
            }
        }
    }
}

// implement Executor
pub struct Executor {
    threads_pool: OnceCell<ThreadsPool>,
}

impl Executor {
    const fn new_const () -> Self {
        Executor {
            threads_pool: OnceCell::new(),
        }
    }

    pub fn new(threads_num: usize) -> Self {
        unsafe {
            THREADS_NUM = if threads_num == 0 { num_cpus::get() } else { threads_num };
        }
        Self::new_const()
    }

    fn threads_pool(&self) -> &ThreadsPool {
        unsafe {
            self.threads_pool.get_or_init_blocking(|| ThreadsPool::new(THREADS_NUM))
        }
    }

    // spawn a Future by pushing it into local_queue
    pub fn spawn(fut: impl Future<Output = ()> + 'static + Send) {
        let task = Arc::new(Task {
            future: RefCell::new(Box::pin(fut)),
            signal: Arc::new(Signal::new()),
        });
        let _ = GLOBAL_EXECUTOR.threads_pool().execute(task);
    }

    // block on to wait for the Future to finish
    pub fn block_on<F: Future>(&self, future: F) -> F::Output {
        let signal = Arc::new(Signal::new());
        let waker = Waker::from(signal.clone());
        let mut cx = Context::from_waker(&waker);
        
        EX.set(self, || {
            let mut main_fut = std::pin::pin!(future);
            loop {
                if let Poll::Ready(output) = main_fut.as_mut().poll(&mut cx) {
                    return output;
                }

                signal.wait();
            }
        })
    }
}

// implement Default for Executor
impl Default for Executor {
    fn default() -> Self {
        Self::new(1)
    }
}