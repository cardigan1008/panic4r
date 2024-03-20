mod offset;

pub fn shl(x: i32) -> i32 {
    x << offset::make_31()
}