use std::ops::{Bound, RangeBounds};

use smallvec::{SmallVec, smallvec};

struct RangeFromExcluded {
    from: usize,
}

impl RangeBounds<usize> for RangeFromExcluded {
    fn start_bound(&self) -> std::ops::Bound<&usize> {
        Bound::Excluded(&self.from)
    }

    fn end_bound(&self) -> std::ops::Bound<&usize> {
        Bound::Unbounded
    }
}

fn main() {
    let mut v: SmallVec<[u8; 8]> = smallvec![1,2,3];

    //should panic but drains 0..0
    let _ = v.drain(..=usize::MAX);
    dbg!(&v);

    //should panic but drains full range
    let _ = v.drain(RangeFromExcluded { from: usize::MAX });
    dbg!(&v);
}