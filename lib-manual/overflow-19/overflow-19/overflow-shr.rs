mod offset;

pub fn shr(x: i32) -> i32 {
    x >> offset::make_31()
}