pub fn one() -> i32 {
    1
}

pub fn index() -> i32 {
    let x = [0];
    let mut y = 0;
    x[one() as usize];
    return y;
}