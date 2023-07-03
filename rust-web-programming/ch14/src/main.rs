use std::time::Instant;
use std::{thread, time};

async fn hello(input_int: i32) -> i32 {
    let five_seconds = time::Duration::from_secs(5);
    tokio::time::sleep(five_seconds).await;
    println!("Hello, world! {}", input_int);
    input_int
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    let now = Instant::now();
    let mut buffer = Vec::new();
    for i in 0..20 {
        let handle = tokio::spawn(async move { hello(i).await });
        buffer.push(handle);
    }
    for i in buffer {
        i.await;
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
