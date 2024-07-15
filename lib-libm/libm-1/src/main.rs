use libm;

fn main() {
    let x = 1.1102230246251565e-16;
    let y = -9.812526705433188e-305;
    let z = 1.0894e-320;
    let result = libm::fma(x, y, z);
    println!("{:?}", result);
}