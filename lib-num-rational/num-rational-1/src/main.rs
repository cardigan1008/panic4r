use num_rational::Ratio;
use num_traits::ops::checked::CheckedDiv;

fn main() {
    let x = Ratio::from(-9223372036854775808_i64);
    x.checked_div(&x);
}