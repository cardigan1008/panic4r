fn main() {
    let re = regex::Regex::new(r"\B|00(?-u)\B").unwrap();
    let text = r"𐾁00000000";
    let rep = r"0𐾁Ű000ο";
    let _ = re.replace(text, rep);
}