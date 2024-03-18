pub mod one;

pub fn sub_assign(mut x: i32) -> i32 {
    x -= one::one();
    
    x
}
