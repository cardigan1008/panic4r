use futures::future::poll_fn;
use std::time::Duration;
use tokio::time; // 1.37.0;

#[tokio::main]
async fn main() {
    let mut timer = time::interval(Duration::MAX);
    //internally tokio calls std::time::Instant::now() + Duration::MAX; which panics
    poll_fn(|cx| timer.poll_tick(cx)).await;
}
