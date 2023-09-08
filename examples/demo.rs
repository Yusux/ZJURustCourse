use my_runtime::executor::Executor;

async fn demo() {
    let (tx1, rx1) = async_channel::bounded::<()>(1);
    let (tx2, rx2) = async_channel::bounded::<()>(1);
    Executor::spawn(demo2(tx1));
    Executor::spawn(demo3(tx2));
    let _ = rx1.recv().await;
    let _ = rx2.recv().await;
    println!("Hello, world!");
}

async fn demo2(tx: async_channel::Sender<()>) {
    println!("This is demo2. Ocuppy CPU by adding 10^9 numbers.");
    for _i in 0..10000 {
        let mut _sum = 0;
        for _j in 0..100000 {
            _sum += 1;
        }
    }
    println!("This is demo2. I will sleep 10 seconds.");
    std::thread::sleep(std::time::Duration::from_secs(10));
    println!("This is demo2. I wake up now.");
    let _ = tx.send(()).await;
}

async fn demo3(tx: async_channel::Sender<()>) {
    println!("This is demo3. Ocuppy CPU by adding 10^9 numbers.");
    for _i in 0..10000 {
        let mut _sum = 0;
        for _j in 0..100000 {
            _sum += 1;
        }
    }
    println!("This is demo3. I will sleep 5 seconds.");
    std::thread::sleep(std::time::Duration::from_secs(5));
    println!("This is demo3. I wake up now.");
    let _ = tx.send(()).await;
}

fn main() {
    let ex = Executor::new(2);
    println!("# Test Multi Tasks without Spin #");
    ex.block_on(demo());
}