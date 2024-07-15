#![allow(dead_code)]
#![warn(clippy::pedantic)]

use num_rational::Ratio;
use num_traits::ops::checked::CheckedDiv;

fn main() {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let x = Ratio::from(0);
    dbg!(x.checked_div(&x));
}