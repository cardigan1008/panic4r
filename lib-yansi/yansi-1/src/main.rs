extern crate yansi;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use yansi::Style;

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn main() {
    let a = Style::new();
    let b = Style::masked();

    assert_eq!(a, b); // passes

    let a_hash = calculate_hash(&a);
    let b_hash = calculate_hash(&b);

    assert_eq!(a_hash, b_hash); // panics
}