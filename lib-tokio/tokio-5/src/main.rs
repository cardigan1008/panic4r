#[tokio::main]
async fn main() {
    // u64::MAX will work as expected.
    let big = std::time::Duration::from_secs(u64::MAX/10);
    tokio::time::sleep(big).await;
}