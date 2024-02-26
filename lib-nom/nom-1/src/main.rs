use nom::{combinator::map, bytes::complete::tag, error::Error, multi::count};

fn main() {
    let parser = map(tag::<_, _, Error<&str>>("abc"), |_| ());
    let result = count(parser, 3)("abcabcabcdef").expect("parsing should succeed");
    println!("{:?}", result);
}