pub fn index() {
    let x: [i32; 2] = [0, 1];
    let a = 3;
    let _ = x[a..];
}


fn main() {
    index();
}