mod offset;

pub fn shr_assign(mut x: i32) -> i32 {
    x >>= offset::make_31();

    x
}
