use nom::character::complete::char;
use nom::error::{convert_error, VerboseError};
use nom::IResult;

fn main() {
    let input = "";

    let result: IResult<_, _, VerboseError<&str>> = char('x')(input);
    match result.unwrap_err() {
        nom::Err::Error(e) | nom::Err::Failure(e) => {
            eprintln!("{:?}", e);
            eprintln!("{}", convert_error(input, e));
        }
        nom::Err::Incomplete(_) => {
            unreachable!();
        }
    }
}