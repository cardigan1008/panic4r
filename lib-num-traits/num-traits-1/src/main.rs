extern crate num_traits;

use num_traits::Num;

fn main() {
    let _ = f32::from_str_radix("™0.2", 10);
}