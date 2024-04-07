#[tokio::main]
async fn main() {
    let big = std::time::Duration::from_secs(u64::MAX/10);
    tokio::time::sleep(big).await;
}
