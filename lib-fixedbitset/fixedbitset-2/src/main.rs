fn main() {
    let v = fixedbitset::FixedBitSet::with_capacity(512usize);
println!("{}", v.count_ones(..));
}
