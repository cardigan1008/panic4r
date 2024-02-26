use std::path::PathBuf;
use url::Url;

fn main() {
    let p = PathBuf::from("c:///");
    let u = Url::from_file_path(p).unwrap();
    println!("url: {:?}", u);
    let r = u.to_file_path();  // <-- Panic happens here
    println!("result: {:?}", r);
}