use syn::Expr;
use std::fs;

fn main() {
    let mut filepath = "./src/in";
    let data = fs::read(filepath).unwrap();
    syn::parse_str::<Expr>(&*String::from_utf8_lossy(&*data));
}