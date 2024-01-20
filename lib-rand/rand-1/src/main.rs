use rand::distributions::WeightedIndex;

pub fn main() {
    WeightedIndex::new(vec![std::f64::NAN, 1.0]).unwrap();
}