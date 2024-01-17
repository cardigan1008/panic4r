use syn::export::quote::quote;
use syn::spanned::Spanned;
use syn::Type;

fn main() {
    let mut t: Type = syn::parse2(quote!(<Self as A>::Q)).unwrap();
    if let Type::Path(ref mut tp) = &mut t {
        tp.path.segments.pop();
        tp.path.segments.pop();
    }
    t.span();
}
