pub fn add_assign(mut x: i32) -> i32 {
    x += 1;
    
    x
}


#[allow(unused)]
fn main() {
    let x: i32 = add_assign(2147483647);
}
