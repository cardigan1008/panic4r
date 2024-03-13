extern crate futures;

use futures::{Future, Stream};

fn main() {
    let e = futures::stream::empty::<Vec<u8>, ()>();
    let mut c = e.concat();
    println!("Poll: {:?}", c.poll());
}