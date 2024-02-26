#[macro_use]
extern crate nom;

fn main() {
    named!(dot(&str) -> &str,
        tag_s!(".")
    );
    dot("點");
}