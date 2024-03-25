pub fn sub_assign(mut x: i32) -> i32 {
    x -= 1;
    
    x
}


#[allow(unused)]
fn main() {
    let x: i32 = sub_assign(-2147483648);
}
