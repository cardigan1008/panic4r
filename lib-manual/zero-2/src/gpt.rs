pub fn div_assign(mut x: i32, y: i32) -> i32 {
    x /= y;

    x
}

#[allow(unused)]
fn main() {
    let x: i32 = div_assign(1, 0);
}
