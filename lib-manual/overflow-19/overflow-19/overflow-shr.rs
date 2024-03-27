mod offset;

pub fn shr(x: u32) -> u32 {
    x >> offset::make_31()
}