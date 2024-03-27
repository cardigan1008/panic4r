mod offset;

pub fn shl_assign(mut x: u32) -> u32 {
    x <<= offset::make_31();

    x
}
