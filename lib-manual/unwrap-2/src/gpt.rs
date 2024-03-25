pub fn unwrap() {
    let x: Option<i32> = None;
    let mut y = 0;

    y = x.unwrap();
}

fn main() {
    unwrap()
}
