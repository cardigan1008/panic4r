use pest::ParserState;

fn main() {
    let input_1 = "❤️";
    let input_2 = "hello world";

    let pairs = pest::state::<(), _>(
        input_1,
        |s| {
            drop(s);
            let state = ParserState::new(input_2);
            state.rule((), |state| state.match_char_by(|c| c.is_ascii()))
        },
    ).unwrap();

    println!("{:?}", pairs);
}