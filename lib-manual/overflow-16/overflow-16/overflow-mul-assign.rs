pub mod two;

pub fn mul_assign(mut x: i32) -> i32 {
    x *= two::two();
    
    x
}