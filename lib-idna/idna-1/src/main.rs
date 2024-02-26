use url::*;
fn main() {
    let mut url = url::Url::parse("file:///d:/test").unwrap();
    url.set_path("d:\\test\\main.foo");
    dbg!(url);
}