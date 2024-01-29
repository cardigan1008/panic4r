use clap::{Command, Arg};

fn main() {
    env_logger::init();
    let command = Command::new("App Name").arg(Arg::new("arg"));
    let positionals = command.get_positionals();
    for arg in positionals {
        println!("arg: {arg}");
    }
}