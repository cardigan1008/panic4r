use syn::__private::quote::quote;

fn main() {
    let x = quote! {
        struct a(fn(mut self,));
    };
    let z: Result<syn::DeriveInput, _> = syn::parse2(x);
}