use futures::stream::{FuturesUnordered, StreamExt};
 
async fn run_loop() {
    async fn test() {
        tokio::spawn(async {
            tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        })
        .await
        .unwrap();
    }
    loop {
        let mut futures = FuturesUnordered::new();
        for _ in 0..4 {
            futures.push(test());
        }
        let _ = futures.next().await;
        drop(futures);
    }
}

#[tokio::main]
async fn main() {
    run_loop().await
}