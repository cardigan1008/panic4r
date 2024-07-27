fn main() {
    let input = "    𓀕 c    ";
    let mut tokens = input
        .parse::<proc_macro2::TokenStream>()
        .unwrap()
        .into_iter();

    let ident1 = tokens.next().unwrap();
    ident1.span().source_text().unwrap();

    let ident2 = tokens.next().unwrap();
    ident2.span().source_text().unwrap();
}
