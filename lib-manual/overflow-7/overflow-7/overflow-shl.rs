fn make_31() -> u32 {
    let mut x = 0;

    for _ in 0..32 {
        x += 1;
    }

    x
}

pub fn shl(x: u32) -> u32 {
    x << make_31()
}