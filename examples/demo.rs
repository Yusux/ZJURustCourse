use my_runtime::executor::Executor;

async fn demo0() {
    let (tx1, rx1) = async_channel::bounded::<()>(1);
    let (tx2, rx2) = async_channel::bounded::<()>(1);
    Executor::spawn(demo2(tx1));
    Executor::spawn(demon(3, 0, tx2));
    let _ = rx1.recv().await;
    let _ = rx2.recv().await;
    println!("Hello, world!");
}

async fn demo2(tx: async_channel::Sender<()>) {
    println!("This is demo2 (Called by demo0). I will Ocuppy CPU by adding 10^9 numbers.");
    for _i in 0..10000 {
        let mut _sum = 0;
        for _j in 0..100000 {
            _sum += 1;
        }
    }
    println!("This is demo2. I will sleep 10 seconds.");
    std::thread::sleep(std::time::Duration::from_secs(10));
    println!("This is demo2. I wake up now.");
    let (tx1, rx1) = async_channel::bounded::<()>(1);
    let (tx2, rx2) = async_channel::bounded::<()>(1);
    Executor::spawn(demon(4, 2, tx1));
    Executor::spawn(demon(5, 2, tx2));
    let _ = rx1.recv().await;
    let _ = rx2.recv().await;
    let _ = tx.send(()).await;
    println!("This is demo2. I Finished.");
}

async fn demon(id: usize, super_id: usize, tx: async_channel::Sender<()>) {
    println!("This is demo{} (Called by demo{}). I will Ocuppy CPU by adding 10^9 numbers.", id, super_id);
    for _i in 0..10000 {
        let mut _sum = 0;
        for _j in 0..100000 {
            _sum += 1;
        }
    }
    println!("This is demo{}. I will sleep 5 seconds.", id);
    std::thread::sleep(std::time::Duration::from_secs(5));
    println!("This is demo{}. I wake up now.", id);
    let _ = tx.send(()).await;
    println!("This is demo{}. I Finished.", id);
}

fn main() {
    let ex = Executor::new(2);
    println!("# Test Multi Tasks without Spin #");
    ex.block_on(demo0());
}