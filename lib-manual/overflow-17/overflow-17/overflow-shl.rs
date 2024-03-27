mod offset;

pub fn shl(x: u32) -> u32 {
    x << offset::make_31()
}