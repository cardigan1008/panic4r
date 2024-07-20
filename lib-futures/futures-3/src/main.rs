use futures::{pin_mut, prelude::*}; // 0.3.6
use tokio; // 0.3.1

#[tokio::main]
async fn main() {
    let s = stream::iter(vec![Ok(1), Ok(2), Ok(3)])
        .try_filter_map(|v| async move { if v == 1 { Err("Ohno!") } else { Ok(Some(v)) } })
        .inspect_err(|err| println!("Error {:?}", err))
        .filter_map(|r| future::ready(r.ok()));
    pin_mut!(s);
    let result = s.next().await;
    println!("{:?}", result);
}