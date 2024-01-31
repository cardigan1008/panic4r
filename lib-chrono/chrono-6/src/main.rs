fn main() {
    chrono::naive::NaiveDateTime::parse_from_str("\u{c}SUN\u{e}\u{3000}\0m@J\u{3000}\0\u{3000}\0m\u{c}!\u{c}\u{b}\u{c}\u{c}\u{c}\u{c}%A\u{c}\u{b}\0SU\u{c}\u{c}",
                                                 "\u{c}\u{c}%A\u{c}\u{b}\0SUN\u{c}\u{c}\u{c}SUNN\u{c}\u{c}\u{c}SUN\u{c}\u{c}!\u{c}\u{b}\u{c}\u{c}\u{c}\u{c}%A\u{c}\u{b}%a");
    let _local1 = chrono::offset::FixedOffset::east_opt(17367308);
    let _local2_param0_helper1 = _local1.unwrap();
    chrono::offset::Offset::fix(&(_local2_param0_helper1));
}
