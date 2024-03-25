pub fn one() -> i32 {
    1
}

pub fn add_assign(mut x: i32) -> i32 {
    x += one();

    x
}

fn main() {
    let _: i32 = add_assign(2147483647);
}
