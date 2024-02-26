use std::iter;
extern crate bytes;

use bytes::{Bytes, BytesMut, Buf, BufMut};
fn main() {
    iter::repeat(b'x')
    .scan(100, |cnt, item| if *cnt >= 1 { *cnt -= 1; Some(item) } else { None })
    .collect::<bytes::Bytes>();
}
