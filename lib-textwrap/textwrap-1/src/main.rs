use textwrap::fill;

fn main() {
    let s = "\u{1b}!Ͽ";
    fill(s, 10);
}
