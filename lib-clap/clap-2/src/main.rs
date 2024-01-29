use clap::{value_parser, Arg, ArgAction, Command};

fn main() {
    let mut matches = Command::new("myprog")
        .arg(
            Arg::new("aopt")
                .short('a')
                .long("aopt")
                .value_parser(value_parser!(usize))
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("bopt")
                .short('b')
                .long("bopt")
                .value_parser(value_parser!(usize))
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("copt")
                .short('c')
                .long("copt")
                .value_parser(value_parser!(usize))
                .action(ArgAction::Append),
        )
        .get_matches_from(vec!["myprog", "-a1", "-b1", "-c1", "-c2", "-b3", "-c3"]);

    let aopt = matches
        .indices_of("aopt")
        .expect("missing aopt indices")
        .collect::<Vec<_>>()
        .into_iter()
        .zip(
            matches
                .remove_many::<usize>("aopt")
                .expect("missing aopt values"),
        )
        .rev()
        .collect::<Vec<_>>();
    let bopt = matches
        .indices_of("bopt")
        .expect("missing bopt indices")
        .collect::<Vec<_>>()
        .into_iter()
        .zip(
            matches
                .remove_many::<usize>("bopt")
                .expect("missing bopt values"),
        )
        .rev()
        .collect::<Vec<_>>();
    println!("{aopt:?} {bopt:?}");
}