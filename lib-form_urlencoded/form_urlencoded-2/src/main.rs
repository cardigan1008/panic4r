use url::Url;

fn main() {
    let url = Url::parse("a:/a/..//a").unwrap();
url.username(); // panic
}
