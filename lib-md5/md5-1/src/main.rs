extern crate md5;

use std::io::prelude::*;

fn main() {
    let data = vec![0; 8 * 1024 * 1024];
    let mut ctx = md5::Context::new();
    let mut len = 0;
    let mut count = 0;
    loop {
        match ctx.write(&data) {
            Ok(n) => {
                len += n;
                count += 1;
            }
            Err(e) => {
                println!("{}", e);
                break;
            }
        }
        println!("#{} written: {}", count, len);
    }
}