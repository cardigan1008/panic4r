#[allow(unused)]
#[allow(arithmetic_overflow)]
pub fn dep() {
    let x: [i32; 2] = [0, 1];
    let y: i32 = x[1];
    let z: i32 = 2147483647;
    let dep = z + y;
}
