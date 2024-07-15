extern crate phf;
extern crate phf_builder;
fn main() {
    let map: phf::map::Map<String, String> = phf_builder::Map::new().build();
    map.get("aaa");
}