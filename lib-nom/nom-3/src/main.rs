use nom::{
    bytes::complete::take_while,
    character::complete::newline,
    error::{convert_error, ParseError, VerboseError},
    sequence::terminated,
    Err, IResult,
};

pub fn bug<'a, E: ParseError<&'a str>>(s: &'a str) -> IResult<&'a str, &'a str, E> {
    terminated(
        take_while(|c| c == 'a'),
        newline, // opt(newline) fixes it
    )(s)
}

fn main() {
    match bug::<VerboseError<&str>>("aaaa") {
        Err(Err::Error(e)) => println!("Error: {}", convert_error("aaaa", e)), // This should not panic!
        _ => unreachable!(),
    }
}