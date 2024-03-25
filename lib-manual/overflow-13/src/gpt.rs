pub fn two() -> i32 {
    2
}

pub fn mul(x: i32) -> i32 {
    x * two()
}

fn main() {
    let _: i32 = mul(2147483647);
}
