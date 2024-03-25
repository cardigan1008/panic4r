pub fn one() -> i32 {
    1
}

pub fn sub_assign(mut x: i32) -> i32 {
    x -= one();
    
    x
}

fn main() {
    let _: i32 = sub_assign(-2147483648);
}
