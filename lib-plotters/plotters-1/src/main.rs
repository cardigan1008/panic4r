use plotters::prelude::*;

fn main() {
    let mut backend = BitMapBackend::new("test.png", (256, 256));
    backend.draw_line((1, 0), (100, 0), &RED).unwrap();
}