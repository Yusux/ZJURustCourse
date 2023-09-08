use my_runtime::executor::Executor;
use std::sync::Arc;

static SIZE: usize = 1919810;
static COUNT: usize = 800;

async fn calculate_vec_sum_father(v: Arc<Vec<f64>>, threads: usize) -> f64 {
    // spawn `threads` tasks to calculate the sum of each slice
    let (tx, rx) = async_channel::bounded::<f64>(threads);
    let tx = Arc::new(tx);
    let mut pos: usize = 0;
    let cnt = SIZE / threads;
    for _i in 0..threads-1 {
        Executor::spawn(calculate_vec_sum_child(v.clone(), pos, cnt, tx.clone()));
        pos += cnt;
    }
    Executor::spawn(calculate_vec_sum_child(v.clone(), pos, v.len() - pos, tx.clone()));

    // wait for the result of each task
    let mut sum: f64 = 0.0;
    for _i in 0..threads {
        sum += rx.recv().await.unwrap();
    }

    sum
}

async fn calculate_vec_sum_child(v: Arc<Vec<f64>>, pos: usize, cnt: usize, tx: Arc<async_channel::Sender<f64>>) {
    // println!("calculate_vec_sum_child");
    let sum = v[pos as usize..(pos + cnt) as usize].iter().sum();
    let _ = tx.send(sum).await;
}

fn dummy_calculate() {
    println!("# Test Async Calculation #");

    let v: Vec<f64> = vec![114.514; SIZE];
    let v = Arc::new(v);

    println!("Single Thread: ");
    let mut sum = 0.0;
    let start = std::time::Instant::now();
    for _i in 0..COUNT {
        sum = v.iter().sum();
    }
    let end = std::time::Instant::now();
    println!("sum = {}, time = {:?}", sum, end.duration_since(start));

    println!("Multi Threads: ");
    let mut sum: f64 = 0.0;
    let start = std::time::Instant::now();
    let ex = Executor::new(4);
    for _i in 0..COUNT {
        sum = ex.block_on(calculate_vec_sum_father(v.clone(), 4));
    }
    let end = std::time::Instant::now();
    println!("sum = {}, time = {:?}", sum, end.duration_since(start));
}

fn main() {
    dummy_calculate();
}