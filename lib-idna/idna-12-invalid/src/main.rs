extern crate url;

use url::Url;

fn main() {
    let mut url = Url::from_file_path("/").unwrap();
    url.path_segments_mut().unwrap().pop_if_empty();
}