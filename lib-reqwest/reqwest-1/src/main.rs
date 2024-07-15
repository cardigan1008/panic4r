use tokio; // 引入 tokio
use reqwest::get; // 引入 reqwest

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let text = reqwest::get("https://www.cloudflare.com/cdn-cgi/trace")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("{text}");
}