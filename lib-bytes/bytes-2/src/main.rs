use bytes::{Bytes, BytesMut, Buf, BufMut};
use std::io::Cursor;
fn main() {
    let a = Cursor::new(Bytes::from(&b"hello"[..]));
    let b = Cursor::new(Bytes::from(&b"world"[..]));
    let mut out = vec![];
    out.put(a.chain(b).take(8));
}
