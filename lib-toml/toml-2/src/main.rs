use serde::Serialize;
use std::collections::HashMap;
use toml_edit::ser;

#[derive(Serialize, Hash, PartialEq, Eq)]
enum Foo {
    #[serde(rename = "hello")]
    Hello,
}

fn main() {
    let mut map = HashMap::new();
    map.insert(Foo::Hello, true);
    //println!("{}", );
    println!("{}", ser::to_string(&map).unwrap());
}