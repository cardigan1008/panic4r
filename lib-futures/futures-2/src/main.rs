use futures::stream::FuturesUnordered;
use std::future::ready;

fn main() {
    let _ = FuturesUnordered::from_iter(vec![ready(1), ready(2), ready(3), ready(4)])
        .into_iter()
        .take(3)
        .count();
}