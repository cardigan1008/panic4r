use syn::export::Span;
use syn::LitInt;

fn main() {
    let span = Span::call_site();
    LitInt::new("-1", span);
}