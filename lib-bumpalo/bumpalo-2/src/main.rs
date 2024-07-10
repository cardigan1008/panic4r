use bumpalo::*;

fn main() {
    let layout = std::alloc::Layout::from_size_align(1, 0x1000).unwrap();
    bumpalo::Bump::new().alloc_layout(layout);
}