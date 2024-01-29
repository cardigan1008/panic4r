use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(args_conflicts_with_subcommands(true))]
struct Opt {
    #[arg(short, long)]
    foo: bool,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    Bar(Bar),
}

#[derive(Args, Debug)]
struct Bar {
    #[arg(short, long)]
    baz: bool,
}

fn main() {
    let opt = Opt::parse();
    println!("{opt:?}");
}