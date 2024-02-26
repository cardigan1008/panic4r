extern crate url;
use url::Url;
fn main() {
    let chrome = "mailto:"; // or "chrome:"
    let url = Url::parse(chrome).ok().unwrap();
    url.cannot_be_a_base();
}