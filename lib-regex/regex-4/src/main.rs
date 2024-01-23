fn main() {
    let re = regex::Regex::new(r"c.*d\z").unwrap();
    println!("{:?}", re.shortest_match_at("ababcd", 4));
}