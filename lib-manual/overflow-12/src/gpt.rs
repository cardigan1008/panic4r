pub fn one() -> i32 {
    1
}

pub fn sub(x: i32) -> i32 {
    x - one()
}

fn main() {
    let _: i32 = sub(-2147483648);
}
