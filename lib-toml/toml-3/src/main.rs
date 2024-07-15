use toml_edit::{Document, Key, Item, Value, Formatted};

fn main() {
    let mut doc = r#"aaaaaa = 1"#.parse::<Document>().unwrap();
    let key = "bbb".parse::<Key>().unwrap();
    let mut table = doc.as_table_mut();
    table.insert_formatted(&key, Item::Value(Value::Integer(Formatted::new(2))));
    
    assert_eq!(doc.to_string(), "aaaaaa = 1\nbbb = 2\n");
}