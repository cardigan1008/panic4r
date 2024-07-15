use serde_yaml;

fn main() {
    let _ = serde_yaml::from_str::<serde_yaml::Value>(">\n@");
}
