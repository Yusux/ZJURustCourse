mod waker;
mod task;
use task::block_on;
use async_channel;
use async_std::task::spawn;

async fn demo() {
    let (tx1, rx1) = async_channel::bounded::<()>(1);
    let (tx2, rx2) = async_channel::bounded::<()>(1);
    spawn(demo2(tx1));
    spawn(demo3(tx2));
    let _ = rx1.recv().await;
    let _ = rx2.recv().await;
    println!("Hello, world!");
}

async fn demo2(tx: async_channel::Sender<()>) {
    println!("This is demo2. I will sleep 10 seconds.");
    std::thread::sleep(std::time::Duration::from_secs(10));
    println!("This is demo2. I wake up now.");
    let _ = tx.send(()).await;
}

async fn demo3(tx: async_channel::Sender<()>) {
    println!("This is demo3. I will sleep 20 seconds.");
    std::thread::sleep(std::time::Duration::from_secs(20));
    println!("This is demo3. I wake up now.");
    let _ = tx.send(()).await;
}

fn main() {
    block_on(demo());
}
