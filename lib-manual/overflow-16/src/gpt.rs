pub fn two() -> i32 {
    2
}

pub fn mul_assign(mut x: i32) -> i32 {
    x *= two();
    
    x
}

#[allow(unused)]
fn main() {
    let x: i32 = mul_assign(1073741824);
}
