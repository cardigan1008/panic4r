pub struct One {
    one: i32,
}

#[allow(unused)]
#[allow(arithmetic_overflow)]
pub fn dep() {
    let x: [i32; 2] = [1, 2];
    let one = One { one: x[1] };
    let y: i32 = 2147483646;
    let dep = y + one.one;
}
