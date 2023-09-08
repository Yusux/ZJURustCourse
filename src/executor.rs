use std::{
    cell::RefCell,
    rc::Rc,
    task::Wake,
    sync::{Arc, Mutex, mpsc},
    collections::VecDeque,
    future::Future,
    task::{Context, Poll, Waker},
    marker::PhantomData,
    thread::JoinHandle,
};

use futures::future::BoxFuture;
use scoped_tls::scoped_thread_local;

use crate::waker::Signal;

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
        EX.with(|ex| {
            ex.local_queue.push(self.clone());
        });
        self.signal.notify();
    }
}

// package a VecDeque as TaskQueue
// use RefCell to make it mutable in a immutable struct
struct TaskQueue {
    queue: RefCell<VecDeque<Arc<Task>>>,
}

impl TaskQueue {
    fn new() -> Self {
        const DEFAULT_CAPACITY: usize = 1024;
        TaskQueue {
            queue: RefCell::new(VecDeque::with_capacity(DEFAULT_CAPACITY)),
        }
    }

    fn new_with_capacity(capacity: usize) -> Self {
        TaskQueue {
            queue: RefCell::new(VecDeque::with_capacity(capacity)),
        }
    }

    fn push(&self, task: Arc<Task>) {
        self.queue.borrow_mut().push_back(task);
    }

    fn pop(&self) -> Option<Arc<Task>> {
        self.queue.borrow_mut().pop_front()
    }
}

// implement Worker to execute Task in a thread
struct Worker {
    wid: usize,
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

        Worker { wid: id, wthread: Some(thread) }
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
        if (max_workers == 0) {
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
    local_queue: TaskQueue,
    threads_pool: ThreadsPool,

    // Disable Send and Sync
    _marker: PhantomData<Rc<()>>,
}

impl Executor {
    pub fn new(threads: usize) -> Self {
        Executor {
            local_queue: TaskQueue::new(),
            threads_pool: ThreadsPool::new(threads),
            _marker: PhantomData,
        }
    }

    // spawn a Future by pushing it into local_queue
    pub fn spawn(fut: impl Future<Output = ()> + 'static + Send) {
        let task = Arc::new(Task {
            future: RefCell::new(Box::pin(fut)),
            signal: Arc::new(Signal::new()),
        });
        EX.with(|ex| {
            ex.local_queue.push(task.clone());
        });
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

                // println!("runnable: {:?}", self.local_queue.queue.borrow().len());
                while let Some(task) = self.local_queue.pop() {
                    // println!("inner while let");
                    // use ThreadsPool to execute Task
                    self.threads_pool.execute(task);
                }
                // println!("runnable: {:?}", self.local_queue.queue.borrow().len());

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
