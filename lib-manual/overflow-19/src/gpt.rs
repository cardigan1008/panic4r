pub fn make_31() -> i32 {
    let mut x = 0;

    for _ in 0..32 {
        x += 1;
    }

    x
}

pub fn shr(x: i32) -> i32 {
    x >> make_31()
}

fn main() {
    let _: i32 = shr(1);
}