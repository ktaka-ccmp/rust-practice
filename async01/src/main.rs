use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

use tokio::runtime::Runtime;
use tokio::task;
use tokio::time::sleep as async_sleep;
use futures::future;

#[tokio::main]
async fn main() {
    let now = Instant::now();
    // async_main01().await;
    // async_main02().await;
    // async_main03().await;
    // async_main04().await;
    // async_main05().await;
    // async_main06().await;
    async_main07().await;
    println!("Elapsed time: {:?}", now.elapsed());
}

async fn async_main01() {
    let handles = vec![
        task::spawn(one_sec_hello()),
        task::spawn(two_sec_hello()),
        task::spawn(three_sec_hello()),
        task::spawn(four_sec_hello()),
        task::spawn(five_sec_hello()),
    ];

    for handle in handles {
        handle.await.unwrap();
    }
}

async fn async_main02() {

    // This will run each future sequentially
    // and will wait for each to complete before starting the next one.
    // This is not efficient as it will take 15 seconds to complete.
    one_sec_hello().await;
    two_sec_hello().await;
    three_sec_hello().await;
    four_sec_hello().await;
    five_sec_hello().await;
}

async fn async_main03() {

    let f1 = one_sec_hello();
    let f2 = two_sec_hello();
    let f3 = three_sec_hello();
    let f4 = four_sec_hello();
    let f5 = five_sec_hello();

    future::join5(f1, f2, f3, f4, f5).await;
}

// This is an alternative way to run multiple futures concurrently
// using `future::join_all` which is more flexible and can handle a dynamic number of futures.
async fn async_main04() {

    use std::pin::Pin;
    use std::future::Future;

    // Pin the futures to ensure they are not moved after being created.
    let f1 = Box::pin(one_sec_hello()) as Pin<Box<dyn Future<Output = ()> + Send>>;
    let f2 = Box::pin(two_sec_hello()) as Pin<Box<dyn Future<Output = ()> + Send>>;
    let f3 = Box::pin(three_sec_hello()) as Pin<Box<dyn Future<Output = ()> + Send>>;
    let f4 = Box::pin(four_sec_hello()) as Pin<Box<dyn Future<Output = ()> + Send>>;
    let f5 = Box::pin(five_sec_hello()) as Pin<Box<dyn Future<Output = ()> + Send>>;

    // This will run all futures concurrently
    future::join_all(vec![f1, f2, f3, f4, f5]).await;
}

async fn async_main05() {
    // This is an alternative way to run multiple futures concurrently
    let f1 = tokio::spawn(one_sec_hello());
    let f2 = tokio::spawn(two_sec_hello());
    let f3 = tokio::spawn(three_sec_hello());
    let f4 = tokio::spawn(four_sec_hello());
    let f5 = tokio::spawn(five_sec_hello());

    future::join_all(vec![f1, f2, f3, f4, f5]).await;
}

async fn async_main06() {
    // This is an alternative way to run multiple futures concurrently
    let handles = vec![
        tokio::spawn(one_sec_hello()),
        tokio::spawn(two_sec_hello()),
        tokio::spawn(three_sec_hello()),
        tokio::spawn(four_sec_hello()),
        tokio::spawn(five_sec_hello()),
    ];
    for handle in handles {
        match handle.await {
            Ok(_) => println!("Task completed successfully"),
            Err(e) => println!("Task failed: {:?}", e),
        }
    }
    println!("All tasks completed");
}

async fn async_main07() {
    tokio::join!(
        one_sec_hello(),
        two_sec_hello(),
        three_sec_hello(),
        four_sec_hello(),
        five_sec_hello()
    );
}


async fn one_sec_hello() {
    println!("Starting one_sec_hello... ");
    async_sleep(Duration::from_secs(1)).await;
    println!("Hello, world! after 1 second");
}

async fn two_sec_hello() {
    println!("Starting two_sec_hello... ");
    async_sleep(Duration::from_secs(2)).await;
    println!("Hello, world! after 2 seconds");
}

async fn three_sec_hello() {
    println!("Starting three_sec_hello... ");
    async_sleep(Duration::from_secs(3)).await;
    println!("Hello, world! after 3 seconds");
}

async fn four_sec_hello() {
    println!("Starting four_sec_hello... ");
    async_sleep(Duration::from_secs(4)).await;
    println!("Hello, world! after 4 seconds");
}

async fn five_sec_hello() {
    println!("Starting five_sec_hello... ");
    async_sleep(Duration::from_secs(5)).await;
    println!("Hello, world! after 5 seconds");
}
