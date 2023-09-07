// Target: implement customed async/await runtime for learning purpose
mod waker;
use waker::Signal;

use std::future::Future;
use std::sync::Arc;
use std::task::{Context, Poll, Waker};
use async_channel;

async fn demo() {
    let (tx, rx) = async_channel::bounded::<()>(1);
    std::thread::spawn(move || {
        // sleep 10 seconds
        std::thread::sleep(std::time::Duration::from_secs(10));
        let _ = tx.send(());
    });
    let _ = rx.recv().await;
    println!("Hello, world!");
}

fn block_on<F: Future>(future: F) -> F::Output {
    let mut fut = std::pin::pin!(future);
    let signal = Arc::new(Signal::new());
    let waker = Waker::from(signal.clone());

    let mut cx = Context::from_waker(&waker);
    loop {
        // println!("poll");
        if let Poll::Ready(output) = fut.as_mut().poll(&mut cx) {
            return output;
        }
        signal.wait();
    }

}

fn main() {
    block_on(demo());
}
