



use std::io::{self, Read};


fn main() {
    let mut input = String::new();
    let result = io::stdin().read_to_string(&mut input);
    if result.is_ok() {
/*
        {
            let mut headers = [httparse::EMPTY_HEADER; 16];
            let mut req = httparse::Request::new(&mut headers);
            req.parse(input.as_bytes());
        }
*/

        {
            let mut headers = [httparse::EMPTY_HEADER; 16];
            let mut res = httparse::Response::new(&mut headers);
            res.parse(input.as_bytes());
        }
    }
}
