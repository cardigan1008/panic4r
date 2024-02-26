extern crate url;
use url::*;
fn main() {
    let url = Url::parse("file:#").unwrap();
    let a=url.fragment(); // panic
}
