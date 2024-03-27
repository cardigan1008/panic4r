mod offset;

pub fn shr_assign(mut x: u32) -> u32 {
    x >>= offset::make_31();

    x
}
