fn main() {
    enum Void {}
    let sv = smallvec::SmallVec::<[Void; 8]>::new();
}