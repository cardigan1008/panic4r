use bytemuck;
fn main() {
    let zst: [u32; 0] = [];
    let _result = bytemuck::bytes_of(&zst);  // Panic: entered unreachable code
}