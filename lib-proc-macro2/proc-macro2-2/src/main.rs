fn main() {
    let s = std::str::from_utf8(b"b\'\xc2\x86  \x00\x00\x00^\"").unwrap();
    if let Ok(token_stream) = s.parse::<proc_macro2::TokenStream>() {
        for _ in token_stream { }
    }
}