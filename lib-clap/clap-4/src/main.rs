use clap::*;

fn main() {
    let c = Arg::new("c").short('c').conflicts_with("gr");
    let app = Command::new("test")
        .group(ArgGroup::new("gr").args(&["a", "b"]))
        .arg(Arg::new("a").short('a'))
        .arg(Arg::new("b").short('b'))
        .arg(&c);
        
    let res =app.clone().try_get_matches_from(vec!["cmd", "-c", "-a"]);
    if let Err(e) = res {
        assert_eq!(e.kind(), clap::error::ErrorKind::ArgumentConflict);
        e.print().unwrap();
    } else {
        unreachable!();
    }

    println!("conflicts: {:?}", app.get_arg_conflicts_with(&c));
}