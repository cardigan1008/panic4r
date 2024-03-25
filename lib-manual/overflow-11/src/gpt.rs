pub fn one() -> i32 {
    1
}

pub fn add(x: i32) -> i32 {
    x + one()
}

fn main() {
    let _: i32 = add(2147483647);
}
