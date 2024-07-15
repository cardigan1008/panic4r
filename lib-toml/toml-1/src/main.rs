use toml_edit::{Document, Key, Item, Value, Formatted};

fn main() {
    let mut doc = r#"a=1"#.parse::<Document>().unwrap();
    let key = "bbbbbb".parse::<Key>().unwrap();
    let mut table = doc.as_table_mut();
    table.insert_formatted(&key, Item::Value(Value::Integer(Formatted::new(2))));
    
    assert_eq!(doc.to_string(), "a=1\nbbbbbb = 2\n");
}