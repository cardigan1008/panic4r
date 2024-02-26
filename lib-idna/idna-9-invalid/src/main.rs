extern crate url;
use url::Url;
fn main() {
    let _url = url::Url::parse("file://./foo");
}