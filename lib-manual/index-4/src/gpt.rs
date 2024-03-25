pub fn one() -> i32 {
    1
}

#[allow(unused)]
pub fn index() -> i32 {
    let x = [0];
    let mut y = 0;
    let z = x[one() as usize];
    return y;
}

fn main() {
    index();
}
