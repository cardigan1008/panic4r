pub fn one() -> usize {
    1
}
pub fn index() -> i32 {
    let x = [0];
    if one() < x.len() && x[one()] > 0 {
        return 1;
    } else {
        return 0;
    }
}
