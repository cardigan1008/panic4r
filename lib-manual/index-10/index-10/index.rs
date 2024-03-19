pub fn get_slice(x: Vec<i32>) -> Vec<i32> {
    x
}

pub fn index() {
    let x: [i32; 2] = [0, 1];
    match get_slice(x[1..0].to_vec()) {
        _ => {}
    }
}
