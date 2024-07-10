extern crate num_bigint;
extern crate num_traits;
extern crate num_integer;
use num_bigint::BigUint;
use num_bigint::Sign::{Minus, NoSign, Plus};
use num_bigint::{BigInt, ToBigInt};
use num_integer::Integer;
use num_traits::{Float, FromPrimitive, Num, One, Pow, Signed, ToPrimitive, Zero};

fn check(a: isize, b: isize, c: isize) {
    let big_a: BigInt = FromPrimitive::from_isize(a).unwrap();
    let big_b: BigInt = FromPrimitive::from_isize(b).unwrap();
    let big_c: BigInt = FromPrimitive::from_isize(c).unwrap();

    assert_eq!(big_a.lcm(&big_b), big_c);
}


fn main() {
    check(0, 0, 0);
}
