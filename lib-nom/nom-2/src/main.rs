use nom::*;
named!(parser<&str, &str>, take_while_m_n!(1, 1, |c| c == 'A' || c == '😃'));

fn main() {
    assert_eq!(parser("A!"), Ok(("!", "A")));
    assert_eq!(parser("😃!"), Ok(("!", "😃")));
}