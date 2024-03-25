fn make_31() -> i32 {
    let mut x = 0;

    for _ in 0..32 {
        x += 1;
    }

    x
}

pub fn shl(x: i32) -> i32 {
    x << make_31()
}

#[allow(unused)]
fn main() {
    let x: i32 = shl(1);
}
