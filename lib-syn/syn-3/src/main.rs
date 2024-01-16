use syn::{LitFloat, parse_str};

fn main() {
    let test = "9e99e99";
    parse_str::<LitFloat>(test);
}