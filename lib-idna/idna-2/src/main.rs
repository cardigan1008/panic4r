use url::*;
fn main() {
    let url = dbg!(Url::parse("         m:/          /%2E.//     \\           ").unwrap());
    let encoded = url.as_str();
    let reparsed = Url::parse(encoded).unwrap();
}