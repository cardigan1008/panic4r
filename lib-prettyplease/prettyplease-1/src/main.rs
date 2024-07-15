use prettyplease::unparse;
use syn::parse_file;

fn main() {
    let code = r#"
        fn example() {
            let Ok(_) = some_result else {
                println!("Error!");
            };
        }
    "#;

    let syntax = syn::parse_file(code).expect("Failed to parse file");
    let formatted = unparse(&syntax);
    println!("{}", formatted);
}
