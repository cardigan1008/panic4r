pub fn one() -> i32 {
    let x = [1, 2, 3, 4];

    x[2]
}

fn main() {
    assert!(one() == 1);
}
