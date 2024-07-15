use smallvec::SmallVec;


fn main() {
    // This test should only panic once, and not double panic,
    // which would mean a double drop
    struct DropPanic;

    impl Drop for DropPanic {
        fn drop(&mut self) {
            panic!("drop");
        }
    }

    let mut v = SmallVec::<[_; 1]>::new();
    v.push(DropPanic);
}