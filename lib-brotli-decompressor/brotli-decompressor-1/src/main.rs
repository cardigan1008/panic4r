use std::io;
use brotli::BrotliDecompress;

fn main() {
    let failing = vec![17, 27, 17, 17, 17, 17, 149, 149, 37, 208, 17, 17, 149, 149, 236, 234, 0, 0, 0, 8, 0, 0, 32, 44, 32, 44, 255, 255, 255, 255, 61, 4, 103, 122, 0, 0, 247, 115, 105, 114, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 252, 0, 0, 32, 0, 0, 0, 0, 33, 225, 252];
    match brotli::BrotliDecompress(&mut failing.as_slice(), &mut io::stdout()) {
        Ok(_) => {
            println!("Ok, world!");
        }
        Err(_) => {
            println!("Error, world!")
        }
    }
}