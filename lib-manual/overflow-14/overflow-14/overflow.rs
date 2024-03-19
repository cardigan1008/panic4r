pub mod one;

pub fn add_assign(mut x: i32) -> i32 {
    x += one::one();

    x
}
