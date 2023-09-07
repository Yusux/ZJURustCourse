use crate::waker::Signal;

use std::cell::RefCell;
use futures::future::BoxFuture;
use std::task::Wake;
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::VecDeque;
use scoped_tls::scoped_thread_local;

use std::future::Future;
use std::task::{Context, Poll, Waker};

scoped_thread_local!(pub static RUNNABLE: Mutex<VecDeque<Arc<Task>>>);
scoped_thread_local!(pub static SIGNAL: Arc<Signal>);

pub struct Task {
    future: RefCell<BoxFuture<'static, ()>>,
    signal: Arc<Signal>,
}

unsafe impl Send for Task {}

unsafe impl Sync for Task {}

impl Wake for Task {
    fn wake(self: Arc<Self>) {
        RUNNABLE.with(|runnable| {
            runnable.lock().unwrap().push_back(self.clone());
        });
        self.signal.notify();
    }
}

pub fn block_on<F: Future>(future: F) -> F::Output {
    let mut main_fut = std::pin::pin!(future);
    let signal = Arc::new(Signal::new());
    let waker = Waker::from(signal.clone());
    let mut cx = Context::from_waker(&waker);
    let runnable = Mutex::new(VecDeque::with_capacity(1024));
    SIGNAL.set(&signal, || {
        RUNNABLE.set(&runnable, || {
            loop {
                if let Poll::Ready(output) = main_fut.as_mut().poll(&mut cx) {
                    return output;
                }
                while let Some(task) = runnable.lock().unwrap().pop_front() {
                    let waker = Waker::from(task.clone());
                    let mut cx = Context::from_waker(&waker);
                    let _ = task.future.borrow_mut().as_mut().poll(&mut cx);
                }
                signal.wait();
            }
        })
    })
}