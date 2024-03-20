pub fn explicit() {
    let x = [1, 2, 3, 4];

    if x.len() >= 5 {
        unreachable!();
    }
}