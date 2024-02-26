extern crate url;
use url::*;
fn main() {
    let mut url = Url::parse("https://example.net/hello").unwrap();
    url.set_host(None).unwrap();
}
