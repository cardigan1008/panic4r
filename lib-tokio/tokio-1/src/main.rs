use std::time::Duration;
use futures::StreamExt;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tokio::time::pause();
    let mut queue = tokio_util::time::DelayQueue::new();
    for _ in 0..2 {
        tokio::time::advance(Duration::from_millis(1 << 35)).await;
        queue.insert(0, Duration::from_millis(0));
        queue.next().await;
    }
    queue.insert(1, Duration::from_millis(1));
}