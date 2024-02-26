extern crate bytes;
use bytes::*;

fn main() {
    let mut buf = RingBuf::new(8);
    buf.write(&[1, 2, 3, 4, 5, 6, 7]).unwrap();
    buf.mark();
    buf.write(&[8]).unwrap();
    buf.reset();

    let mut buf2 = [0; 8];
    buf.read(&mut buf2).unwrap();
    assert_eq!(buf2, [1, 2, 3, 4, 5, 6, 7, 8]);
}