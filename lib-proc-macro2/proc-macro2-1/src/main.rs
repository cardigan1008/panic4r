fn main() {
    proc_macro2::Span::call_site();
    println!("ran anyway");
}