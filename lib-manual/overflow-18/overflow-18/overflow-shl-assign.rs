mod offset;

pub fn shl_assign(mut x: i32) -> i32 {
    x <<= offset::make_31();

    x
}
